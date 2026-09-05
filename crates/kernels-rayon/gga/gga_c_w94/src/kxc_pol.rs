//! GGA_C_W94 kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_w94.c`
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
pub fn gga_c_w94_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = (f64x8::splat(0.0)).simd_lt(t4);
            let t6 = ((t5).select(t4, -t4));
            let t7 = (f64x8::splat(1e-10)).simd_lt(t6);
            let t8 = ((t7).select(t6, f64x8::splat(1e-10)));
            let t9 = (simd::cbrt(t8));
            let t10 = t9 * t9;
            let t12 = -t10 * t8 + f64x8::splat(1.0);
            let t13 = ((t12).sqrt());
            let t15 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t16 = ((t15).sqrt());
            let t17 = t16 * t15;
            let t18 = t2 * t2;
            let t19 = t18 * t18;
            let t20 = f64x8::splat(1.0) / t19;
            let t22 = (simd::cbrt(t2));
            let t24 = f64x8::splat(1.0) / t22 / t2;
            let t25 = t16 * t24;
            let t26 = (simd::pow(t25, f64x8::splat(1.0) / f64x8::splat(16.0)));
            let t27 = t26 * t26;
            let t28 = t27 * t26;
            let t31 = t18 * t2;
            let t32 = f64x8::splat(1.0) / t31;
            let t35 = f64x8::splat(M_CBRT3);
            let t37 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t38 = t35 * t37;
            let t39 = f64x8::splat(M_CBRT4);
            let t40 = t39 * t39;
            let t45 = f64x8::splat(11.8) + f64x8::splat(0.15067) * t28 * t17 * t20 + f64x8::splat(0.01102) * t15 * t32 + t38 * t40 / t22 / f64x8::splat(4.0);
            let t46 = f64x8::splat(1.0) / t45;
            let tzk0 = -t13 * t46;
            acc_zk = tzk0;
            let t48 = f64x8::splat(1.0) / t13;
            let t49 = t2 * t48;
            let t50 = t46 * t10;
            let t51 = f64x8::splat(1.0) / t18;
            let t52 = t1 * t51;
            let t53 = t3 - t52;
            let t55 = ((t5).select(t53, -t53));
            let t56 = ((t7).select(t55, f64x8::splat(0.0)));
            let t60 = t2 * t13;
            let t61 = t45 * t45;
            let t62 = f64x8::splat(1.0) / t61;
            let t63 = t22 * t22;
            let t65 = f64x8::splat(1.0) / t63 / t18;
            let t67 = t28 * t15 * t65;
            let t68 = t67 * t16;
            let t70 = f64x8::splat(1.0) / t22 / t18;
            let t78 = -f64x8::splat(0.6403475) * t68 * t70 - f64x8::splat(0.03306) * t15 * t20 - t38 * t40 * t24 / f64x8::splat(12.0);
            let t80 = t60 * t62 * t78;
            let tvrho0 = tzk0 + f64x8::splat(5.0) / f64x8::splat(6.0) * t49 * t50 * t56 + t80;
            acc_vrho_0 = tvrho0;
            let t81 = -t3 - t52;
            let t83 = ((t5).select(t81, -t81));
            let t84 = ((t7).select(t83, f64x8::splat(0.0)));
            let tvrho1 = tzk0 + f64x8::splat(5.0) / f64x8::splat(6.0) * t49 * t50 * t84 + t80;
            acc_vrho_1 = tvrho1;
            let t88 = f64x8::splat(1.0) / t16;
            let t89 = t67 * t88;
            let t90 = t89 * t24;
            let t93 = f64x8::splat(0.2401303125) * t90 + f64x8::splat(0.01102) * t32;
            let tvsigma0 = t60 * t62 * t93;
            acc_vsigma_0 = tvsigma0;
            let t97 = f64x8::splat(0.480260625) * t90 + f64x8::splat(0.02204) * t32;
            let tvsigma1 = t60 * t62 * t97;
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
            let t99 = t48 * t46;
            let t100 = t10 * t56;
            let t101 = t99 * t100;
            let t103 = t13 * t62;
            let t105 = f64x8::splat(2.0) * t103 * t78;
            let t107 = f64x8::splat(1.0) / t13 / t12;
            let t108 = t2 * t107;
            let t109 = t9 * t8;
            let t110 = t46 * t109;
            let t111 = t56 * t56;
            let t115 = t49 * t62;
            let t116 = t100 * t78;
            let t117 = t115 * t116;
            let t119 = f64x8::splat(1.0) / t9;
            let t120 = t46 * t119;
            let t124 = t1 * t32;
            let t126 = -f64x8::splat(2.0) * t51 + f64x8::splat(2.0) * t124;
            let t128 = ((t5).select(t126, -t126));
            let t129 = ((t7).select(t128, f64x8::splat(0.0)));
            let t134 = f64x8::splat(1.0) / t61 / t45;
            let t135 = t78 * t78;
            let t138 = f64x8::splat(2.0) * t60 * t134 * t135;
            let t139 = t28 * t25;
            let t140 = t139 * t15;
            let t142 = f64x8::splat(1.0) / t63 / t19;
            let t146 = f64x8::splat(1.0) / t22 / t31;
            let t149 = t19 * t2;
            let t150 = f64x8::splat(1.0) / t149;
            let t156 = f64x8::splat(1.8676802083333333) * t140 * t142 + f64x8::splat(1.4941441666666666) * t68 * t146 + f64x8::splat(0.13224) * t15 * t150 + t38 * t40 * t70 / f64x8::splat(9.0);
            let t158 = t60 * t62 * t156;
            let tv2rho20 = f64x8::splat(5.0) / f64x8::splat(3.0) * t101 + t105 + f64x8::splat(25.0) / f64x8::splat(36.0) * t108 * t110 * t111 - f64x8::splat(5.0) / f64x8::splat(3.0) * t117 + f64x8::splat(5.0) / f64x8::splat(9.0) * t49 * t120 * t111 + f64x8::splat(5.0) / f64x8::splat(6.0) * t49 * t50 * t129 - t138 + t158;
            acc_v2rho2_0 = tv2rho20;
            let t160 = t10 * t84;
            let t161 = t99 * t160;
            let t163 = t108 * t46;
            let t164 = t109 * t84;
            let t165 = t164 * t56;
            let t168 = t160 * t78;
            let t169 = t115 * t168;
            let t171 = t49 * t46;
            let t172 = t119 * t84;
            let t173 = t172 * t56;
            let t176 = f64x8::splat(2.0) * t124;
            let t177 = ((t5).select(t176, -t176));
            let t178 = ((t7).select(t177, f64x8::splat(0.0)));
            let tv2rho21 = f64x8::splat(5.0) / f64x8::splat(6.0) * t101 + t105 + f64x8::splat(5.0) / f64x8::splat(6.0) * t161 + f64x8::splat(25.0) / f64x8::splat(36.0) * t163 * t165 - f64x8::splat(5.0) / f64x8::splat(6.0) * t169 + f64x8::splat(5.0) / f64x8::splat(9.0) * t171 * t173 + f64x8::splat(5.0) / f64x8::splat(6.0) * t49 * t50 * t178 - f64x8::splat(5.0) / f64x8::splat(6.0) * t117 - t138 + t158;
            acc_v2rho2_1 = tv2rho21;
            let t184 = t84 * t84;
            let t193 = f64x8::splat(2.0) * t51 + f64x8::splat(2.0) * t124;
            let t195 = ((t5).select(t193, -t193));
            let t196 = ((t7).select(t195, f64x8::splat(0.0)));
            let tv2rho22 = f64x8::splat(5.0) / f64x8::splat(3.0) * t161 + t105 + f64x8::splat(25.0) / f64x8::splat(36.0) * t108 * t110 * t184 - f64x8::splat(5.0) / f64x8::splat(3.0) * t169 + f64x8::splat(5.0) / f64x8::splat(9.0) * t49 * t120 * t184 + f64x8::splat(5.0) / f64x8::splat(6.0) * t49 * t50 * t196 - t138 + t158;
            acc_v2rho2_2 = tv2rho22;
            let t200 = t103 * t93;
            let t201 = t93 * t10;
            let t202 = t201 * t56;
            let t205 = t134 * t93;
            let t208 = f64x8::splat(2.0) * t60 * t205 * t78;
            let t210 = f64x8::splat(1.0) / t63 / t31;
            let t211 = t139 * t210;
            let t213 = t89 * t70;
            let t216 = -f64x8::splat(0.700380078125) * t211 - f64x8::splat(0.32017375) * t213 - f64x8::splat(0.03306) * t20;
            let t218 = t60 * t62 * t216;
            let tv2rhosigma0 = t200 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t202 - t208 + t218;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let t219 = t103 * t97;
            let t220 = t97 * t10;
            let t221 = t220 * t56;
            let t224 = t134 * t97;
            let t227 = f64x8::splat(2.0) * t60 * t224 * t78;
            let t231 = -f64x8::splat(1.40076015625) * t211 - f64x8::splat(0.6403475) * t213 - f64x8::splat(0.06612) * t20;
            let t233 = t60 * t62 * t231;
            let tv2rhosigma1 = t219 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t221 - t227 + t233;
            acc_v2rhosigma_1 = tv2rhosigma1;
            let tv2rhosigma2 = tv2rhosigma0;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t234 = t201 * t84;
            let tv2rhosigma3 = t200 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t234 - t208 + t218;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let t237 = t220 * t84;
            let tv2rhosigma4 = t219 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t237 - t227 + t233;
            acc_v2rhosigma_4 = tv2rhosigma4;
            let tv2rhosigma5 = tv2rhosigma3;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t240 = t93 * t93;
            let t244 = f64x8::splat(1.0) / t15;
            let t245 = t139 * t244;
            let t246 = t245 * t65;
            let t248 = f64x8::splat(1.0) / t17;
            let t249 = t67 * t248;
            let t250 = t249 * t24;
            let t252 = f64x8::splat(0.262642529296875) * t246 - f64x8::splat(0.12006515625) * t250;
            let tv2sigma20 = -f64x8::splat(2.0) * t60 * t134 * t240 + t60 * t62 * t252;
            acc_v2sigma2_0 = tv2sigma20;
            let t260 = f64x8::splat(0.52528505859375) * t246 - f64x8::splat(0.2401303125) * t250;
            let tv2sigma21 = -f64x8::splat(2.0) * t60 * t224 * t93 + t60 * t62 * t260;
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = tv2sigma20;
            acc_v2sigma2_2 = tv2sigma22;
            let t263 = t97 * t97;
            let t269 = f64x8::splat(1.0505701171875) * t246 - f64x8::splat(0.480260625) * t250;
            let tv2sigma23 = -f64x8::splat(2.0) * t60 * t134 * t263 + t60 * t62 * t269;
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = tv2sigma21;
            acc_v2sigma2_4 = tv2sigma24;
            let tv2sigma25 = tv2sigma22;
            acc_v2sigma2_5 = tv2sigma25;
            let t272 = t10 * t129;
            let t273 = t99 * t272;
            let t275 = t107 * t46;
            let t276 = t109 * t111;
            let t277 = t275 * t276;
            let t279 = t13 * t134;
            let t281 = f64x8::splat(6.0) * t279 * t135;
            let t282 = t28 * t17;
            let t283 = t19 * t31;
            let t284 = f64x8::splat(1.0) / t283;
            let t288 = f64x8::splat(1.0) / t63 / t149;
            let t292 = f64x8::splat(1.0) / t22 / t19;
            let t295 = t19 * t18;
            let t296 = f64x8::splat(1.0) / t295;
            let t302 = -f64x8::splat(2.9571603298611113) * t282 * t284 - f64x8::splat(13.073761458333333) * t140 * t288 - f64x8::splat(4.980480555555555) * t68 * t292 - f64x8::splat(0.6612) * t15 * t296 - f64x8::splat(7.0) / f64x8::splat(27.0) * t38 * t40 * t146;
            let t304 = t60 * t62 * t302;
            let t305 = t48 * t62;
            let t306 = t305 * t116;
            let t308 = t119 * t111;
            let t309 = t99 * t308;
            let t311 = t12 * t12;
            let t313 = f64x8::splat(1.0) / t13 / t311;
            let t314 = t2 * t313;
            let t315 = t8 * t8;
            let t316 = t46 * t315;
            let t317 = t111 * t56;
            let t321 = t1 * t20;
            let t323 = f64x8::splat(6.0) * t32 - f64x8::splat(6.0) * t321;
            let t325 = ((t5).select(t323, -t323));
            let t326 = ((t7).select(t325, f64x8::splat(0.0)));
            let t330 = t61 * t61;
            let t331 = f64x8::splat(1.0) / t330;
            let t332 = t135 * t78;
            let t335 = f64x8::splat(6.0) * t60 * t331 * t332;
            let t339 = f64x8::splat(6.0) * t60 * t134 * t78 * t156;
            let t340 = t108 * t62;
            let t341 = t276 * t78;
            let t342 = t340 * t341;
            let t344 = t46 * t9;
            let t348 = t109 * t56;
            let t349 = t348 * t129;
            let t352 = t272 * t78;
            let t353 = t115 * t352;
            let t355 = t100 * t156;
            let t356 = t115 * t355;
            let t358 = f64x8::splat(1.0) / t109;
            let t359 = t46 * t358;
            let t364 = t119 * t56 * t129;
            let t368 = f64x8::splat(3.0) * t103 * t156;
            let t369 = t49 * t134;
            let t370 = t100 * t135;
            let t371 = t369 * t370;
            let t373 = t308 * t78;
            let t374 = t115 * t373;
            let tv3rho30 = f64x8::splat(5.0) / f64x8::splat(2.0) * t273 + f64x8::splat(25.0) / f64x8::splat(12.0) * t277 - t281 + t304 - f64x8::splat(5.0) * t306 + f64x8::splat(5.0) / f64x8::splat(3.0) * t309 + f64x8::splat(125.0) / f64x8::splat(72.0) * t314 * t316 * t317 + f64x8::splat(5.0) / f64x8::splat(6.0) * t49 * t50 * t326 + t335 - t339 - f64x8::splat(25.0) / f64x8::splat(12.0) * t342 + f64x8::splat(25.0) / f64x8::splat(18.0) * t108 * t344 * t317 + f64x8::splat(25.0) / f64x8::splat(12.0) * t163 * t349 - f64x8::splat(5.0) / f64x8::splat(2.0) * t353 - f64x8::splat(5.0) / f64x8::splat(2.0) * t356 - f64x8::splat(5.0) / f64x8::splat(27.0) * t49 * t359 * t317 + f64x8::splat(5.0) / f64x8::splat(3.0) * t171 * t364 + t368 + f64x8::splat(5.0) * t371 - f64x8::splat(5.0) / f64x8::splat(3.0) * t374;
            acc_v3rho3_0 = tv3rho30;
            let t376 = t305 * t168;
            let t379 = f64x8::splat(10.0) / f64x8::splat(9.0) * t99 * t173;
            let t381 = f64x8::splat(25.0) / f64x8::splat(18.0) * t275 * t165;
            let t382 = f64x8::splat(2.0) * t32;
            let t383 = f64x8::splat(6.0) * t321;
            let t384 = t382 - t383;
            let t386 = ((t5).select(t384, -t384));
            let t387 = ((t7).select(t386, f64x8::splat(0.0)));
            let t392 = t10 * t178;
            let t394 = f64x8::splat(5.0) / f64x8::splat(3.0) * t99 * t392;
            let t398 = t9 * t84;
            let t399 = t398 * t111;
            let t402 = t368 - f64x8::splat(5.0) / f64x8::splat(3.0) * t376 + t379 + t381 + f64x8::splat(5.0) / f64x8::splat(6.0) * t49 * t50 * t387 - t339 - f64x8::splat(10.0) / f64x8::splat(3.0) * t306 - t281 + t394 + f64x8::splat(5.0) / f64x8::splat(6.0) * t273 + f64x8::splat(25.0) / f64x8::splat(36.0) * t277 + t304 + f64x8::splat(5.0) / f64x8::splat(9.0) * t309 + t335 + f64x8::splat(25.0) / f64x8::splat(18.0) * t163 * t399;
            let t403 = t160 * t135;
            let t404 = t369 * t403;
            let t406 = t358 * t84;
            let t407 = t406 * t111;
            let t415 = t109 * t178;
            let t416 = t415 * t56;
            let t419 = t164 * t129;
            let t422 = t314 * t46;
            let t423 = t315 * t84;
            let t424 = t423 * t111;
            let t427 = t392 * t78;
            let t429 = f64x8::splat(5.0) / f64x8::splat(3.0) * t115 * t427;
            let t430 = t160 * t156;
            let t431 = t115 * t430;
            let t433 = t119 * t178;
            let t434 = t433 * t56;
            let t437 = t172 * t129;
            let t440 = t56 * t78;
            let t441 = t164 * t440;
            let t443 = f64x8::splat(25.0) / f64x8::splat(18.0) * t340 * t441;
            let t444 = t172 * t440;
            let t446 = f64x8::splat(10.0) / f64x8::splat(9.0) * t115 * t444;
            let t447 = f64x8::splat(5.0) / f64x8::splat(3.0) * t404 - f64x8::splat(5.0) / f64x8::splat(27.0) * t171 * t407 - f64x8::splat(25.0) / f64x8::splat(36.0) * t342 - f64x8::splat(5.0) / f64x8::splat(6.0) * t353 - f64x8::splat(5.0) / f64x8::splat(3.0) * t356 + f64x8::splat(10.0) / f64x8::splat(3.0) * t371 - f64x8::splat(5.0) / f64x8::splat(9.0) * t374 + f64x8::splat(25.0) / f64x8::splat(18.0) * t163 * t416 + f64x8::splat(25.0) / f64x8::splat(36.0) * t163 * t419 + f64x8::splat(125.0) / f64x8::splat(72.0) * t422 * t424 - t429 - f64x8::splat(5.0) / f64x8::splat(6.0) * t431 + f64x8::splat(10.0) / f64x8::splat(9.0) * t171 * t434 + f64x8::splat(5.0) / f64x8::splat(9.0) * t171 * t437 - t443 - t446;
            let tv3rho31 = t402 + t447;
            acc_v3rho3_1 = tv3rho31;
            let t448 = -t382 - t383;
            let t450 = ((t5).select(t448, -t448));
            let t451 = ((t7).select(t450, f64x8::splat(0.0)));
            let t457 = t109 * t184;
            let t458 = t275 * t457;
            let t460 = t119 * t184;
            let t461 = t99 * t460;
            let t463 = t10 * t196;
            let t464 = t99 * t463;
            let t467 = t368 + f64x8::splat(5.0) / f64x8::splat(6.0) * t49 * t50 * t451 - f64x8::splat(10.0) / f64x8::splat(3.0) * t376 + t379 + t381 - t339 - f64x8::splat(5.0) / f64x8::splat(3.0) * t306 - t281 + f64x8::splat(25.0) / f64x8::splat(36.0) * t458 + f64x8::splat(5.0) / f64x8::splat(9.0) * t461 + f64x8::splat(5.0) / f64x8::splat(6.0) * t464 + t394 + t304 + t335 + f64x8::splat(10.0) / f64x8::splat(3.0) * t404;
            let t468 = t457 * t78;
            let t469 = t340 * t468;
            let t471 = t9 * t184;
            let t472 = t471 * t56;
            let t475 = t164 * t178;
            let t478 = t315 * t184;
            let t479 = t478 * t56;
            let t482 = t460 * t78;
            let t483 = t115 * t482;
            let t485 = t358 * t184;
            let t486 = t485 * t56;
            let t489 = t172 * t178;
            let t492 = t463 * t78;
            let t493 = t115 * t492;
            let t495 = t119 * t196;
            let t496 = t495 * t56;
            let t499 = t109 * t196;
            let t500 = t499 * t56;
            let t506 = -f64x8::splat(25.0) / f64x8::splat(36.0) * t469 + f64x8::splat(25.0) / f64x8::splat(18.0) * t163 * t472 + f64x8::splat(25.0) / f64x8::splat(18.0) * t163 * t475 + f64x8::splat(125.0) / f64x8::splat(72.0) * t422 * t479 - f64x8::splat(5.0) / f64x8::splat(9.0) * t483 - f64x8::splat(5.0) / f64x8::splat(27.0) * t171 * t486 + f64x8::splat(10.0) / f64x8::splat(9.0) * t171 * t489 - f64x8::splat(5.0) / f64x8::splat(6.0) * t493 + f64x8::splat(5.0) / f64x8::splat(9.0) * t171 * t496 + f64x8::splat(25.0) / f64x8::splat(36.0) * t163 * t500 - f64x8::splat(5.0) / f64x8::splat(6.0) * t356 + f64x8::splat(5.0) / f64x8::splat(3.0) * t371 - t429 - f64x8::splat(5.0) / f64x8::splat(3.0) * t431 - t443 - t446;
            let tv3rho32 = t467 + t506;
            acc_v3rho3_2 = tv3rho32;
            let t511 = t184 * t84;
            let t516 = -f64x8::splat(6.0) * t32 - f64x8::splat(6.0) * t321;
            let t518 = ((t5).select(t516, -t516));
            let t519 = ((t7).select(t518, f64x8::splat(0.0)));
            let t530 = t164 * t196;
            let t536 = t172 * t196;
            let tv3rho33 = -t281 + t304 + t335 - t339 - f64x8::splat(5.0) * t376 + f64x8::splat(25.0) / f64x8::splat(12.0) * t458 + f64x8::splat(5.0) / f64x8::splat(3.0) * t461 + f64x8::splat(5.0) / f64x8::splat(2.0) * t464 + f64x8::splat(125.0) / f64x8::splat(72.0) * t314 * t316 * t511 + f64x8::splat(5.0) / f64x8::splat(6.0) * t49 * t50 * t519 - f64x8::splat(5.0) / f64x8::splat(2.0) * t431 - f64x8::splat(25.0) / f64x8::splat(12.0) * t469 - f64x8::splat(5.0) / f64x8::splat(3.0) * t483 - f64x8::splat(5.0) / f64x8::splat(2.0) * t493 + f64x8::splat(25.0) / f64x8::splat(18.0) * t108 * t344 * t511 + f64x8::splat(25.0) / f64x8::splat(12.0) * t163 * t530 - f64x8::splat(5.0) / f64x8::splat(27.0) * t49 * t359 * t511 + f64x8::splat(5.0) / f64x8::splat(3.0) * t171 * t536 + t368 + f64x8::splat(5.0) * t404;
            acc_v3rho3_3 = tv3rho33;
            let t540 = t305 * t202;
            let t542 = t93 * t78;
            let t544 = f64x8::splat(4.0) * t279 * t542;
            let t546 = f64x8::splat(2.0) * t103 * t216;
            let t547 = t93 * t109;
            let t548 = t547 * t111;
            let t551 = t201 * t440;
            let t552 = t369 * t551;
            let t554 = t216 * t10;
            let t555 = t554 * t56;
            let t556 = t115 * t555;
            let t558 = t93 * t119;
            let t559 = t558 * t111;
            let t562 = t201 * t129;
            let t568 = f64x8::splat(6.0) * t60 * t331 * t93 * t135;
            let t569 = t134 * t216;
            let t572 = f64x8::splat(4.0) * t60 * t569 * t78;
            let t575 = f64x8::splat(2.0) * t60 * t205 * t156;
            let t577 = t28 * t296 * t16;
            let t579 = t139 * t142;
            let t581 = t89 * t146;
            let t584 = f64x8::splat(1.1089351236979166) * t577 + f64x8::splat(3.501900390625) * t579 + f64x8::splat(0.7470720833333333) * t581 + f64x8::splat(0.13224) * t150;
            let t586 = t60 * t62 * t584;
            let tv3rho2sigma0 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t540 - t544 + t546 - f64x8::splat(25.0) / f64x8::splat(36.0) * t340 * t548 + f64x8::splat(10.0) / f64x8::splat(3.0) * t552 - f64x8::splat(5.0) / f64x8::splat(3.0) * t556 - f64x8::splat(5.0) / f64x8::splat(9.0) * t115 * t559 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t562 + t568 - t572 - t575 + t586;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let t587 = t305 * t221;
            let t589 = t97 * t78;
            let t591 = f64x8::splat(4.0) * t279 * t589;
            let t593 = f64x8::splat(2.0) * t103 * t231;
            let t594 = t97 * t109;
            let t595 = t594 * t111;
            let t599 = t369 * t220 * t440;
            let t601 = t231 * t10;
            let t602 = t601 * t56;
            let t603 = t115 * t602;
            let t605 = t97 * t119;
            let t606 = t605 * t111;
            let t609 = t220 * t129;
            let t612 = t331 * t97;
            let t615 = f64x8::splat(6.0) * t60 * t612 * t135;
            let t616 = t134 * t231;
            let t619 = f64x8::splat(4.0) * t60 * t616 * t78;
            let t622 = f64x8::splat(2.0) * t60 * t224 * t156;
            let t627 = f64x8::splat(2.217870247395833) * t577 + f64x8::splat(7.00380078125) * t579 + f64x8::splat(1.4941441666666666) * t581 + f64x8::splat(0.26448) * t150;
            let t629 = t60 * t62 * t627;
            let tv3rho2sigma1 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t587 - t591 + t593 - f64x8::splat(25.0) / f64x8::splat(36.0) * t340 * t595 + f64x8::splat(10.0) / f64x8::splat(3.0) * t599 - f64x8::splat(5.0) / f64x8::splat(3.0) * t603 - f64x8::splat(5.0) / f64x8::splat(9.0) * t115 * t606 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t609 + t615 - t619 - t622 + t629;
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let tv3rho2sigma2 = tv3rho2sigma0;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t631 = t305 * t234;
            let t633 = t84 * t56;
            let t634 = t547 * t633;
            let t637 = t84 * t78;
            let t638 = t201 * t637;
            let t639 = t369 * t638;
            let t641 = t554 * t84;
            let t642 = t115 * t641;
            let t644 = t558 * t633;
            let t647 = t201 * t178;
            let tv3rho2sigma3 = -f64x8::splat(5.0) / f64x8::splat(6.0) * t540 - t544 + t546 - f64x8::splat(5.0) / f64x8::splat(6.0) * t631 - f64x8::splat(25.0) / f64x8::splat(36.0) * t340 * t634 + f64x8::splat(5.0) / f64x8::splat(3.0) * t639 - f64x8::splat(5.0) / f64x8::splat(6.0) * t642 - f64x8::splat(5.0) / f64x8::splat(9.0) * t115 * t644 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t647 + f64x8::splat(5.0) / f64x8::splat(3.0) * t552 + t568 - t572 - t575 - f64x8::splat(5.0) / f64x8::splat(6.0) * t556 + t586;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let t653 = t305 * t237;
            let t659 = t369 * t220 * t637;
            let t661 = t601 * t84;
            let t662 = t115 * t661;
            let t667 = t220 * t178;
            let tv3rho2sigma4 = -f64x8::splat(5.0) / f64x8::splat(6.0) * t587 - t591 + t593 - f64x8::splat(5.0) / f64x8::splat(6.0) * t653 - f64x8::splat(25.0) / f64x8::splat(36.0) * t340 * t594 * t633 + f64x8::splat(5.0) / f64x8::splat(3.0) * t659 - f64x8::splat(5.0) / f64x8::splat(6.0) * t662 - f64x8::splat(5.0) / f64x8::splat(9.0) * t115 * t605 * t633 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t667 + f64x8::splat(5.0) / f64x8::splat(3.0) * t599 + t615 - t619 - t622 - f64x8::splat(5.0) / f64x8::splat(6.0) * t603 + t629;
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let tv3rho2sigma5 = tv3rho2sigma3;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t673 = t547 * t184;
            let t678 = t558 * t184;
            let t681 = t201 * t196;
            let tv3rho2sigma6 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t631 - t544 + t546 - f64x8::splat(25.0) / f64x8::splat(36.0) * t340 * t673 + f64x8::splat(10.0) / f64x8::splat(3.0) * t639 - f64x8::splat(5.0) / f64x8::splat(3.0) * t642 - f64x8::splat(5.0) / f64x8::splat(9.0) * t115 * t678 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t681 + t568 - t572 - t575 + t586;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let t685 = t594 * t184;
            let t690 = t605 * t184;
            let t693 = t220 * t196;
            let tv3rho2sigma7 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t653 - t591 + t593 - f64x8::splat(25.0) / f64x8::splat(36.0) * t340 * t685 + f64x8::splat(10.0) / f64x8::splat(3.0) * t659 - f64x8::splat(5.0) / f64x8::splat(3.0) * t662 - f64x8::splat(5.0) / f64x8::splat(9.0) * t115 * t690 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t693 + t615 - t619 - t622 + t629;
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let tv3rho2sigma8 = tv3rho2sigma6;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t697 = f64x8::splat(2.0) * t279 * t240;
            let t698 = t240 * t10;
            let t699 = t698 * t56;
            let t702 = t331 * t240;
            let t705 = f64x8::splat(6.0) * t60 * t702 * t78;
            let t708 = f64x8::splat(4.0) * t60 * t205 * t216;
            let t709 = t103 * t252;
            let t710 = t252 * t10;
            let t711 = t710 * t56;
            let t714 = t134 * t252;
            let t717 = f64x8::splat(2.0) * t60 * t714 * t78;
            let t718 = t28 * t88;
            let t719 = t718 * t150;
            let t721 = t245 * t210;
            let t723 = t249 * t70;
            let t725 = -f64x8::splat(0.4158506713867188) * t719 - f64x8::splat(0.3501900390625) * t721 + f64x8::splat(0.160086875) * t723;
            let t727 = t60 * t62 * t725;
            let tv3rhosigma20 = -t697 + f64x8::splat(5.0) / f64x8::splat(3.0) * t369 * t699 + t705 - t708 + t709 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t711 - t717 + t727;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let t728 = t97 * t93;
            let t730 = f64x8::splat(2.0) * t279 * t728;
            let t734 = t60 * t331;
            let t735 = t728 * t78;
            let t737 = f64x8::splat(6.0) * t734 * t735;
            let t740 = f64x8::splat(2.0) * t60 * t616 * t93;
            let t743 = f64x8::splat(2.0) * t60 * t224 * t216;
            let t744 = t103 * t260;
            let t745 = t260 * t10;
            let t746 = t745 * t56;
            let t749 = t134 * t260;
            let t752 = f64x8::splat(2.0) * t60 * t749 * t78;
            let t756 = -f64x8::splat(0.8317013427734375) * t719 - f64x8::splat(0.700380078125) * t721 + f64x8::splat(0.32017375) * t723;
            let t758 = t60 * t62 * t756;
            let tv3rhosigma21 = -t730 + f64x8::splat(5.0) / f64x8::splat(3.0) * t369 * t728 * t100 + t737 - t740 - t743 + t744 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t746 - t752 + t758;
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = tv3rhosigma20;
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let t760 = f64x8::splat(2.0) * t279 * t263;
            let t761 = t263 * t10;
            let t762 = t761 * t56;
            let t765 = t331 * t263;
            let t768 = f64x8::splat(6.0) * t60 * t765 * t78;
            let t771 = f64x8::splat(4.0) * t60 * t224 * t231;
            let t772 = t103 * t269;
            let t773 = t269 * t10;
            let t774 = t773 * t56;
            let t777 = t134 * t269;
            let t780 = f64x8::splat(2.0) * t60 * t777 * t78;
            let t784 = -f64x8::splat(1.663402685546875) * t719 - f64x8::splat(1.40076015625) * t721 + f64x8::splat(0.6403475) * t723;
            let t786 = t60 * t62 * t784;
            let tv3rhosigma23 = -t760 + f64x8::splat(5.0) / f64x8::splat(3.0) * t369 * t762 + t768 - t771 + t772 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t774 - t780 + t786;
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = tv3rhosigma21;
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let tv3rhosigma25 = tv3rhosigma22;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t787 = t698 * t84;
            let t790 = t710 * t84;
            let tv3rhosigma26 = -t697 + f64x8::splat(5.0) / f64x8::splat(3.0) * t369 * t787 + t705 - t708 + t709 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t790 - t717 + t727;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let t796 = t745 * t84;
            let tv3rhosigma27 = -t730 + f64x8::splat(5.0) / f64x8::splat(3.0) * t369 * t728 * t160 + t737 - t740 - t743 + t744 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t796 - t752 + t758;
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = tv3rhosigma26;
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let t799 = t761 * t84;
            let t802 = t773 * t84;
            let tv3rhosigma29 = -t760 + f64x8::splat(5.0) / f64x8::splat(3.0) * t369 * t799 + t768 - t771 + t772 - f64x8::splat(5.0) / f64x8::splat(6.0) * t115 * t802 - t780 + t786;
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = tv3rhosigma27;
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let tv3rhosigma211 = tv3rhosigma28;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t805 = t240 * t93;
            let t813 = t28 * t248 * t20;
            let t815 = t15 * t15;
            let t816 = f64x8::splat(1.0) / t815;
            let t817 = t139 * t816;
            let t818 = t817 * t65;
            let t821 = f64x8::splat(1.0) / t16 / t815;
            let t822 = t67 * t821;
            let t823 = t822 * t24;
            let t825 = f64x8::splat(0.15594400177001952) * t813 - f64x8::splat(0.3939637939453125) * t818 + f64x8::splat(0.180097734375) * t823;
            let tv3sigma30 = -f64x8::splat(6.0) * t60 * t205 * t252 + f64x8::splat(6.0) * t60 * t331 * t805 + t60 * t62 * t825;
            acc_v3sigma3_0 = tv3sigma30;
            let t840 = f64x8::splat(0.31188800354003904) * t813 - f64x8::splat(0.787927587890625) * t818 + f64x8::splat(0.36019546875) * t823;
            let tv3sigma31 = -f64x8::splat(2.0) * t60 * t224 * t252 + f64x8::splat(6.0) * t60 * t612 * t240 + t60 * t62 * t840 - f64x8::splat(4.0) * t60 * t749 * t93;
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = tv3sigma30;
            acc_v3sigma3_2 = tv3sigma32;
            let t855 = f64x8::splat(0.6237760070800781) * t813 - f64x8::splat(1.57585517578125) * t818 + f64x8::splat(0.7203909375) * t823;
            let tv3sigma33 = -f64x8::splat(4.0) * t60 * t224 * t260 + t60 * t62 * t855 + f64x8::splat(6.0) * t60 * t765 * t93 - f64x8::splat(2.0) * t60 * t777 * t93;
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = tv3sigma31;
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = tv3sigma32;
            acc_v3sigma3_5 = tv3sigma35;
            let t858 = t263 * t97;
            let t868 = f64x8::splat(1.2475520141601562) * t813 - f64x8::splat(3.1517103515625) * t818 + f64x8::splat(1.440781875) * t823;
            let tv3sigma36 = -f64x8::splat(6.0) * t60 * t224 * t269 + f64x8::splat(6.0) * t60 * t331 * t858 + t60 * t62 * t868;
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = tv3sigma33;
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = tv3sigma34;
            acc_v3sigma3_8 = tv3sigma38;
            let tv3sigma39 = tv3sigma35;
            acc_v3sigma3_9 = tv3sigma39;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
