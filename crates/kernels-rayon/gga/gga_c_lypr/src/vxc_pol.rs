//! GGA_C_LYPR vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lypr.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_lypr_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_m1: f64,
    param_omega: f64,
    param_d: f64,
    param_m2: f64,
    param_b: f64,
    param_c: f64,
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_m1 = f64x8::splat(param_m1);
    let param_omega = f64x8::splat(param_omega);
    let param_d = f64x8::splat(param_d);
    let param_m2 = f64x8::splat(param_m2);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_a = f64x8::splat(param_a);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = param_m1 * param_omega;
            let t2 = v_rho0 + v_rho1;
            let t3 = (simd::cbrt(t2));
            let t4 = f64x8::splat(1.0) / t3;
            let t6 = (simd::erfc(t1 * t4));
            let t7 = v_rho0 - v_rho1;
            let t8 = t7 * t7;
            let t9 = t2 * t2;
            let t10 = f64x8::splat(1.0) / t9;
            let t12 = -t8 * t10 + f64x8::splat(1.0);
            let t13 = t6 * t12;
            let t15 = param_d * t4 + f64x8::splat(1.0);
            let t16 = f64x8::splat(1.0) / t15;
            let t18 = param_m2 * param_omega;
            let t20 = (simd::erfc(t18 * t4));
            let t21 = t20 * param_b;
            let t23 = (simd::exp(-param_c * t4));
            let t24 = t23 * t16;
            let t26 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t27 = t3 * t3;
            let t29 = f64x8::splat(1.0) / t27 / t9;
            let t30 = t26 * t29;
            let t32 = param_d * t16 + param_c;
            let t33 = t32 * t4;
            let t35 = f64x8::splat(47.0) - f64x8::splat(7.0) * t33;
            let t38 = t12 * t35 / f64x8::splat(72.0) - f64x8::splat(2.0) / f64x8::splat(3.0);
            let t40 = f64x8::splat(M_CBRT3);
            let t41 = t40 * t40;
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = t41 * t44;
            let t46 = f64x8::splat(1.0) / t2;
            let t47 = t7 * t46;
            let t48 = f64x8::splat(1.0) + t47;
            let t49 = (t48).simd_le(zeta_threshold);
            let t50 = zeta_threshold * zeta_threshold;
            let t51 = (simd::cbrt(zeta_threshold));
            let t52 = t51 * t51;
            let t53 = t52 * t50;
            let t54 = t48 * t48;
            let t55 = (simd::cbrt(t48));
            let t56 = t55 * t55;
            let t57 = t56 * t54;
            let t58 = ((t49).select(t53, t57));
            let t59 = f64x8::splat(1.0) - t47;
            let t60 = (t59).simd_le(zeta_threshold);
            let t61 = t59 * t59;
            let t62 = (simd::cbrt(t59));
            let t63 = t62 * t62;
            let t64 = t63 * t61;
            let t65 = ((t60).select(t53, t64));
            let t66 = t58 + t65;
            let t70 = f64x8::splat(M_CBRT2);
            let t71 = t70 * t12;
            let t73 = f64x8::splat(5.0) / f64x8::splat(2.0) - t33 / f64x8::splat(18.0);
            let t74 = v_rho0 * v_rho0;
            let t75 = (simd::cbrt(v_rho0));
            let t76 = t75 * t75;
            let t78 = f64x8::splat(1.0) / t76 / t74;
            let t79 = v_sigma0 * t78;
            let t80 = t79 * t58;
            let t81 = v_rho1 * v_rho1;
            let t82 = (simd::cbrt(v_rho1));
            let t83 = t82 * t82;
            let t85 = f64x8::splat(1.0) / t83 / t81;
            let t86 = v_sigma2 * t85;
            let t87 = t86 * t65;
            let t88 = t80 + t87;
            let t89 = t73 * t88;
            let t92 = t33 - f64x8::splat(11.0);
            let t94 = t52 * t50 * zeta_threshold;
            let t97 = ((t49).select(t94, t56 * t54 * t48));
            let t101 = ((t60).select(t94, t63 * t61 * t59));
            let t103 = t86 * t101 + t79 * t97;
            let t104 = t92 * t103;
            let t109 = ((t49).select(t50, t54));
            let t110 = t109 * v_sigma2;
            let t111 = t85 * t65;
            let t114 = ((t60).select(t50, t61));
            let t115 = t114 * v_sigma0;
            let t116 = t78 * t58;
            let t122 = -t30 * t38 - f64x8::splat(3.0) / f64x8::splat(20.0) * t45 * t12 * t66 + t71 * t89 / f64x8::splat(32.0) + t71 * t104 / f64x8::splat(576.0) - t70 * (f64x8::splat(2.0) / f64x8::splat(3.0) * t80 + f64x8::splat(2.0) / f64x8::splat(3.0) * t87 - t110 * t111 / f64x8::splat(4.0) - t115 * t116 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let t123 = t24 * t122;
            let t125 = param_b * t23;
            let t126 = ((f64x8::splat(M_PI)).sqrt());
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t16 * t127;
            let t130 = t125 * t128 * param_m2;
            let t131 = param_m2 * param_m2;
            let t132 = param_omega * param_omega;
            let t134 = f64x8::splat(1.0) / t27;
            let t136 = (simd::exp(-t131 * t132 * t134));
            let t137 = param_omega * t136;
            let t138 = t4 * t12;
            let t142 = t47 / f64x8::splat(6.0);
            let t143 = f64x8::splat(7.0) / f64x8::splat(6.0) + t142;
            let t144 = t143 * v_sigma0;
            let t145 = t70 * t78;
            let t146 = t145 * t58;
            let t149 = f64x8::splat(7.0) / f64x8::splat(6.0) - t142;
            let t150 = t149 * v_sigma2;
            let t151 = t70 * t85;
            let t152 = t151 * t65;
            let t155 = f64x8::splat(7.0) / f64x8::splat(6.0) * t30 - f64x8::splat(7.0) / f64x8::splat(48.0) * t70 * t88 + t144 * t146 / f64x8::splat(8.0) + t150 * t152 / f64x8::splat(8.0);
            let tzk0 = param_a * (-t13 * t16 + t21 * t123 + t130 * t137 * t138 * t155 / f64x8::splat(6.0));
            acc_zk = tzk0;
            let t161 = t2 * param_a;
            let t162 = param_m1 * param_m1;
            let t165 = (simd::exp(-t162 * t132 * t134));
            let t166 = t127 * t165;
            let t167 = t166 * param_m1;
            let t169 = f64x8::splat(1.0) / t3 / t2;
            let t170 = param_omega * t169;
            let t171 = t12 * t16;
            let t174 = f64x8::splat(2.0) / f64x8::splat(3.0) * t167 * t170 * t171;
            let t175 = t7 * t10;
            let t176 = t9 * t2;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t8 * t177;
            let t180 = -f64x8::splat(2.0) * t175 + f64x8::splat(2.0) * t178;
            let t181 = t6 * t180;
            let t183 = t15 * t15;
            let t184 = f64x8::splat(1.0) / t183;
            let t185 = t184 * param_d;
            let t186 = t185 * t169;
            let t188 = t13 * t186 / f64x8::splat(3.0);
            let t189 = t127 * t136;
            let t190 = t189 * t18;
            let t191 = t169 * param_b;
            let t194 = f64x8::splat(2.0) / f64x8::splat(3.0) * t190 * t191 * t123;
            let t195 = t21 * param_c;
            let t196 = t169 * t23;
            let t197 = t16 * t122;
            let t200 = t195 * t196 * t197 / f64x8::splat(3.0);
            let t201 = t21 * t23;
            let t202 = t184 * t122;
            let t203 = param_d * t169;
            let t206 = t201 * t202 * t203 / f64x8::splat(3.0);
            let t208 = f64x8::splat(1.0) / t27 / t176;
            let t209 = t26 * t208;
            let t211 = f64x8::splat(8.0) / f64x8::splat(3.0) * t209 * t38;
            let t213 = param_d * param_d;
            let t214 = t213 * t184;
            let t216 = f64x8::splat(1.0) / t27 / t2;
            let t219 = t32 * t169 - t214 * t216;
            let t220 = f64x8::splat(7.0) / f64x8::splat(3.0) * t219;
            let t221 = t12 * t220;
            let t223 = t180 * t35 / f64x8::splat(72.0) + t221 / f64x8::splat(72.0);
            let t228 = t56 * t48;
            let t229 = t46 - t175;
            let t230 = t228 * t229;
            let t232 = ((t49).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t230));
            let t233 = t63 * t59;
            let t234 = -t229;
            let t235 = t233 * t234;
            let t237 = ((t60).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t235));
            let t238 = t232 + t237;
            let t242 = t70 * t180;
            let t245 = t219 / f64x8::splat(54.0);
            let t246 = t245 * t88;
            let t248 = t71 * t246 / f64x8::splat(32.0);
            let t251 = f64x8::splat(1.0) / t76 / t74 / v_rho0;
            let t252 = v_sigma0 * t251;
            let t253 = t252 * t58;
            let t255 = t79 * t232;
            let t256 = t86 * t237;
            let t257 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t253 + t255 + t256;
            let t258 = t73 * t257;
            let t264 = -t219 / f64x8::splat(3.0);
            let t265 = t264 * t103;
            let t267 = t71 * t265 / f64x8::splat(576.0);
            let t272 = ((t49).select(f64x8::splat(0.0), f64x8::splat(11.0) / f64x8::splat(3.0) * t57 * t229));
            let t276 = ((t60).select(f64x8::splat(0.0), f64x8::splat(11.0) / f64x8::splat(3.0) * t64 * t234));
            let t278 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t252 * t97 + t79 * t272 + t86 * t276;
            let t279 = t92 * t278;
            let t287 = ((t49).select(f64x8::splat(0.0), f64x8::splat(2.0) * t48 * t229));
            let t288 = t287 * v_sigma2;
            let t291 = t85 * t237;
            let t296 = ((t60).select(f64x8::splat(0.0), f64x8::splat(2.0) * t59 * t234));
            let t297 = t296 * v_sigma0;
            let t300 = t251 * t58;
            let t303 = t78 * t232;
            let t309 = t211 - t30 * t223 - f64x8::splat(3.0) / f64x8::splat(20.0) * t45 * t180 * t66 - f64x8::splat(3.0) / f64x8::splat(20.0) * t45 * t12 * t238 + t242 * t89 / f64x8::splat(32.0) + t248 + t71 * t258 / f64x8::splat(32.0) + t242 * t104 / f64x8::splat(576.0) + t267 + t71 * t279 / f64x8::splat(576.0) - t70 * (-f64x8::splat(16.0) / f64x8::splat(9.0) * t253 + f64x8::splat(2.0) / f64x8::splat(3.0) * t255 + f64x8::splat(2.0) / f64x8::splat(3.0) * t256 - t288 * t111 / f64x8::splat(4.0) - t110 * t291 / f64x8::splat(4.0) - t297 * t116 / f64x8::splat(4.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t115 * t300 - t115 * t303 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let t310 = t24 * t309;
            let t312 = param_b * param_c;
            let t315 = t312 * t216 * t23 * t16;
            let t316 = t127 * param_m2;
            let t317 = t316 * param_omega;
            let t318 = t136 * t12;
            let t319 = t318 * t155;
            let t320 = t317 * t319;
            let t322 = t315 * t320 / f64x8::splat(18.0);
            let t323 = t184 * t127;
            let t325 = t125 * t323 * param_m2;
            let t326 = t137 * t216;
            let t327 = t12 * t155;
            let t328 = t327 * param_d;
            let t331 = t325 * t326 * t328 / f64x8::splat(18.0);
            let t332 = t131 * param_m2;
            let t334 = t125 * t128 * t332;
            let t335 = t132 * param_omega;
            let t336 = t335 * t10;
            let t339 = t334 * t336 * t319 / f64x8::splat(9.0);
            let t340 = t169 * t12;
            let t344 = t130 * t137 * t340 * t155 / f64x8::splat(18.0);
            let t345 = t4 * t180;
            let t350 = f64x8::splat(28.0) / f64x8::splat(9.0) * t209;
            let t353 = t229 / f64x8::splat(6.0);
            let t354 = t353 * v_sigma0;
            let t357 = t70 * t251;
            let t358 = t357 * t58;
            let t361 = t145 * t232;
            let t364 = -t353;
            let t365 = t364 * v_sigma2;
            let t368 = t151 * t237;
            let t371 = -t350 - f64x8::splat(7.0) / f64x8::splat(48.0) * t70 * t257 + t354 * t146 / f64x8::splat(8.0) - t144 * t358 / f64x8::splat(3.0) + t144 * t361 / f64x8::splat(8.0) + t365 * t152 / f64x8::splat(8.0) + t150 * t368 / f64x8::splat(8.0);
            let t376 = -t174 - t181 * t16 - t188 + t194 + t200 + t206 + t21 * t310 + t322 + t331 + t339 - t344 + t130 * t137 * t345 * t155 / f64x8::splat(6.0) + t130 * t137 * t138 * t371 / f64x8::splat(6.0);
            let tvrho0 = t161 * t376 + tzk0;
            acc_vrho_0 = tvrho0;
            let t379 = f64x8::splat(2.0) * t175 + f64x8::splat(2.0) * t178;
            let t380 = t6 * t379;
            let t384 = t379 * t35 / f64x8::splat(72.0) + t221 / f64x8::splat(72.0);
            let t389 = -t46 - t175;
            let t390 = t228 * t389;
            let t392 = ((t49).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t390));
            let t393 = -t389;
            let t394 = t233 * t393;
            let t396 = ((t60).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t394));
            let t397 = t392 + t396;
            let t401 = t70 * t379;
            let t404 = t79 * t392;
            let t407 = f64x8::splat(1.0) / t83 / t81 / v_rho1;
            let t408 = v_sigma2 * t407;
            let t409 = t408 * t65;
            let t411 = t86 * t396;
            let t412 = t404 - f64x8::splat(8.0) / f64x8::splat(3.0) * t409 + t411;
            let t413 = t73 * t412;
            let t420 = ((t49).select(f64x8::splat(0.0), f64x8::splat(11.0) / f64x8::splat(3.0) * t57 * t389));
            let t426 = ((t60).select(f64x8::splat(0.0), f64x8::splat(11.0) / f64x8::splat(3.0) * t64 * t393));
            let t428 = t79 * t420 - f64x8::splat(8.0) / f64x8::splat(3.0) * t408 * t101 + t86 * t426;
            let t429 = t92 * t428;
            let t437 = ((t49).select(f64x8::splat(0.0), f64x8::splat(2.0) * t48 * t389));
            let t438 = t437 * v_sigma2;
            let t441 = t407 * t65;
            let t444 = t85 * t396;
            let t449 = ((t60).select(f64x8::splat(0.0), f64x8::splat(2.0) * t59 * t393));
            let t450 = t449 * v_sigma0;
            let t453 = t78 * t392;
            let t459 = t211 - t30 * t384 - f64x8::splat(3.0) / f64x8::splat(20.0) * t45 * t379 * t66 - f64x8::splat(3.0) / f64x8::splat(20.0) * t45 * t12 * t397 + t401 * t89 / f64x8::splat(32.0) + t248 + t71 * t413 / f64x8::splat(32.0) + t401 * t104 / f64x8::splat(576.0) + t267 + t71 * t429 / f64x8::splat(576.0) - t70 * (f64x8::splat(2.0) / f64x8::splat(3.0) * t404 - f64x8::splat(16.0) / f64x8::splat(9.0) * t409 + f64x8::splat(2.0) / f64x8::splat(3.0) * t411 - t438 * t111 / f64x8::splat(4.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t110 * t441 - t110 * t444 / f64x8::splat(4.0) - t450 * t116 / f64x8::splat(4.0) - t115 * t453 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let t460 = t24 * t459;
            let t462 = t4 * t379;
            let t469 = t389 / f64x8::splat(6.0);
            let t470 = t469 * v_sigma0;
            let t473 = t145 * t392;
            let t476 = -t469;
            let t477 = t476 * v_sigma2;
            let t480 = t70 * t407;
            let t481 = t480 * t65;
            let t484 = t151 * t396;
            let t487 = -t350 - f64x8::splat(7.0) / f64x8::splat(48.0) * t70 * t412 + t470 * t146 / f64x8::splat(8.0) + t144 * t473 / f64x8::splat(8.0) + t477 * t152 / f64x8::splat(8.0) - t150 * t481 / f64x8::splat(3.0) + t150 * t484 / f64x8::splat(8.0);
            let t492 = -t174 - t380 * t16 - t188 + t194 + t200 + t206 + t21 * t460 + t322 + t331 + t339 - t344 + t130 * t137 * t462 * t155 / f64x8::splat(6.0) + t130 * t137 * t138 * t487 / f64x8::splat(6.0);
            let tvrho1 = t161 * t492 + tzk0;
            acc_vrho_1 = tvrho1;
            let t494 = t29 * t38;
            let t495 = t73 * t78;
            let t496 = t495 * t58;
            let t499 = t92 * t78;
            let t500 = t499 * t97;
            let t504 = t114 * t78;
            let t510 = -t494 + t71 * t496 / f64x8::splat(32.0) + t71 * t500 / f64x8::splat(576.0) - t70 * (f64x8::splat(2.0) / f64x8::splat(3.0) * t116 - t504 * t58 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let t511 = t24 * t510;
            let t513 = f64x8::splat(7.0) / f64x8::splat(6.0) * t29;
            let t515 = t143 * t78;
            let t516 = t70 * t58;
            let t519 = t513 - f64x8::splat(7.0) / f64x8::splat(48.0) * t146 + t515 * t516 / f64x8::splat(8.0);
            let t524 = t21 * t511 + t130 * t137 * t138 * t519 / f64x8::splat(6.0);
            let tvsigma0 = t161 * t524;
            acc_vsigma_0 = tvsigma0;
            let t525 = t16 * t29;
            let t529 = t125 * t128;
            let t530 = t136 * t177;
            let t535 = -f64x8::splat(2.0) * t201 * t525 * t38 + f64x8::splat(7.0) / f64x8::splat(18.0) * t529 * t18 * t530 * t12;
            let tvsigma1 = t161 * t535;
            acc_vsigma_1 = tvsigma1;
            let t536 = t73 * t85;
            let t537 = t536 * t65;
            let t540 = t92 * t85;
            let t541 = t540 * t101;
            let t545 = t109 * t85;
            let t551 = -t494 + t71 * t537 / f64x8::splat(32.0) + t71 * t541 / f64x8::splat(576.0) - t70 * (f64x8::splat(2.0) / f64x8::splat(3.0) * t111 - t545 * t65 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let t552 = t24 * t551;
            let t555 = t149 * t85;
            let t556 = t70 * t65;
            let t559 = t513 - f64x8::splat(7.0) / f64x8::splat(48.0) * t152 + t555 * t556 / f64x8::splat(8.0);
            let t564 = t21 * t552 + t130 * t137 * t138 * t559 / f64x8::splat(6.0);
            let tvsigma2 = t161 * t564;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
