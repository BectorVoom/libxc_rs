//! MGGA_C_RREGTM vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rregtm.c`
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
pub fn mgga_c_rregtm_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t12 = t5 * t7 / t9;
            let t14 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t15 = ((t12).sqrt());
            let t18 = ((t12) * (t12).sqrt());
            let t20 = t2 * t2;
            let t21 = t4 * t4;
            let t22 = t20 * t21;
            let t23 = t9 * t9;
            let t26 = t22 * t6 / t23;
            let t28 = f64x8::splat(3.79785) * t15 + f64x8::splat(0.8969) * t12 + f64x8::splat(0.204775) * t18 + f64x8::splat(0.123235) * t26;
            let t31 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t28;
            let t32 = (simd::ln(t31));
            let t34 = f64x8::splat(0.0621814) * t14 * t32;
            let t35 = v_rho0 - v_rho1;
            let t36 = t35 * t35;
            let t37 = t36 * t36;
            let t38 = t8 * t8;
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t37 * t40;
            let t42 = f64x8::splat(1.0) / t8;
            let t43 = t35 * t42;
            let t44 = f64x8::splat(1.0) + t43;
            let t45 = (t44).simd_le(zeta_threshold);
            let t46 = (simd::cbrt(zeta_threshold));
            let t47 = t46 * zeta_threshold;
            let t48 = (simd::cbrt(t44));
            let t49 = t48 * t44;
            let t50 = ((t45).select(t47, t49));
            let t51 = f64x8::splat(1.0) - t43;
            let t52 = (t51).simd_le(zeta_threshold);
            let t53 = (simd::cbrt(t51));
            let t54 = t53 * t51;
            let t55 = ((t52).select(t47, t54));
            let t56 = t50 + t55 - f64x8::splat(2.0);
            let t57 = f64x8::splat(M_CBRT2);
            let t58 = t57 - f64x8::splat(1.0);
            let t60 = f64x8::splat(1.0) / t58 / f64x8::splat(2.0);
            let t61 = t56 * t60;
            let t63 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t68 = f64x8::splat(7.05945) * t15 + f64x8::splat(1.549425) * t12 + f64x8::splat(0.420775) * t18 + f64x8::splat(0.1562925) * t26;
            let t71 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t68;
            let t72 = (simd::ln(t71));
            let t76 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t81 = f64x8::splat(5.1785) * t15 + f64x8::splat(0.905775) * t12 + f64x8::splat(0.1100325) * t18 + f64x8::splat(0.1241775) * t26;
            let t84 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t81;
            let t85 = (simd::ln(t84));
            let t86 = t76 * t85;
            let t88 = -f64x8::splat(0.0310907) * t63 * t72 + t34 - f64x8::splat(0.0197516734986138) * t86;
            let t89 = t61 * t88;
            let t90 = t41 * t89;
            let t92 = f64x8::splat(0.0197516734986138) * t61 * t86;
            let t93 = (simd::ln(f64x8::splat(2.0)));
            let t94 = f64x8::splat(1.0) - t93;
            let t95 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t97 = t94 / t95;
            let t98 = t46 * t46;
            let t99 = t48 * t48;
            let t100 = ((t45).select(t98, t99));
            let t101 = t53 * t53;
            let t102 = ((t52).select(t98, t101));
            let t104 = t100 / f64x8::splat(2.0) + t102 / f64x8::splat(2.0);
            let t105 = t104 * t104;
            let t106 = t105 * t104;
            let t108 = f64x8::splat(1.0) + f64x8::splat(0.025) * t12;
            let t110 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t12;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t108 * t111;
            let t113 = f64x8::splat(1.0) / t94;
            let t115 = (-t34 + t90 + t92) * t113;
            let t116 = f64x8::splat(1.0) / t106;
            let t117 = t95 * t116;
            let t119 = (simd::exp(-t115 * t117));
            let t120 = t119 - f64x8::splat(1.0);
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t113 * t121;
            let t124 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t125 = t122 * t124;
            let t126 = t112 * t125;
            let t128 = f64x8::splat(1.0) / t9 / t38;
            let t129 = t128 * t57;
            let t130 = f64x8::splat(1.0) / t105;
            let t132 = f64x8::splat(1.0) / t4;
            let t133 = t20 * t132;
            let t134 = t133 * t6;
            let t138 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t126 * t129 * t130 * t134;
            let t139 = ((t138).sqrt().sqrt());
            let t141 = f64x8::splat(1.0) - f64x8::splat(1.0) / t139;
            let t144 = f64x8::splat(1.0) + f64x8::splat(1.0) * t141 * t120;
            let t145 = (simd::ln(t144));
            let t147 = t97 * t106 * t145;
            let t148 = (simd::cbrt(v_rho0));
            let t149 = t148 * t148;
            let t151 = f64x8::splat(1.0) / t149 / v_rho0;
            let t152 = v_tau0 * t151;
            let t153 = t44 / f64x8::splat(2.0);
            let t154 = (simd::cbrt(t153));
            let t155 = t154 * t154;
            let t156 = t155 * t153;
            let t158 = (simd::cbrt(v_rho1));
            let t159 = t158 * t158;
            let t161 = f64x8::splat(1.0) / t159 / v_rho1;
            let t162 = v_tau1 * t161;
            let t163 = t51 / f64x8::splat(2.0);
            let t164 = (simd::cbrt(t163));
            let t165 = t164 * t164;
            let t166 = t165 * t163;
            let t169 = f64x8::splat(1.0) / t23 / t38;
            let t173 = f64x8::splat(M_CBRT6);
            let t174 = (t152 * t156 + t162 * t166 - t124 * t169 / f64x8::splat(8.0)) * t173;
            let t175 = (simd::cbrt(t95));
            let t176 = t175 * t175;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t156 + t166;
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t177 * t179;
            let t182 = f64x8::splat(5.0) / f64x8::splat(9.0) * t174 * t180;
            let t183 = (t182).simd_le(f64x8::splat(1.0));
            let t184 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t187 = t184 / (-t184 + f64x8::splat(0.64));
            let t188 = (-t187).simd_lt(t182);
            let t189 = (t182).simd_lt(-t187);
            let t190 = ((t189).select(t182, -t187));
            let t191 = f64x8::splat(1.0) - t190;
            let t192 = f64x8::splat(1.0) / t191;
            let t195 = (simd::exp(-f64x8::splat(0.64) * t190 * t192));
            let t196 = ((t188).select(f64x8::splat(0.0), t195));
            let t198 = (simd::ln(f64x8::splat(1.4285714285714286) * f64x8::splat(f64::EPSILON)));
            let t201 = (-t198 + f64x8::splat(1.5)) / t198;
            let t202 = (t182).simd_lt(-t201);
            let t203 = ((t202).select(-t201, t182));
            let t204 = f64x8::splat(1.0) - t203;
            let t207 = (simd::exp(f64x8::splat(1.5) / t204));
            let t209 = ((t202).select(f64x8::splat(0.0), -f64x8::splat(0.7) * t207));
            let t210 = ((t183).select(t196, t209));
            let t213 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t15 + f64x8::splat(0.03138525) * t12;
            let t214 = f64x8::splat(1.0) / t213;
            let t217 = (simd::exp(f64x8::splat(1.0) * t214));
            let t218 = t217 - f64x8::splat(1.0);
            let t219 = t173 * t177;
            let t220 = t57 * t57;
            let t221 = t220 * t124;
            let t225 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t219 * t221 * t169;
            let t226 = ((t225).sqrt().sqrt());
            let t228 = f64x8::splat(1.0) - f64x8::splat(1.0) / t226;
            let t230 = t218 * t228 + f64x8::splat(1.0);
            let t231 = (simd::ln(t230));
            let t233 = -f64x8::splat(0.0285764) * t214 + f64x8::splat(0.0285764) * t231;
            let t237 = f64x8::splat(1.0) - f64x8::splat(2.363) * t58 * t56 * t60;
            let t238 = t233 * t237;
            let t239 = t37 * t37;
            let t240 = t239 * t37;
            let t241 = t39 * t39;
            let t242 = t241 * t39;
            let t243 = f64x8::splat(1.0) / t242;
            let t245 = -t240 * t243 + f64x8::splat(1.0);
            let t247 = t238 * t245 - t147 + t34 - t90 - t92;
            let t248 = t210 * t247;
            let tzk0 = -t34 + t90 + t92 + t147 + t248;
            acc_zk = tzk0;
            let t250 = f64x8::splat(1.0) / t9 / t8;
            let t251 = t7 * t250;
            let t253 = t5 * t251 * t32;
            let t254 = f64x8::splat(0.0011073470983333333) * t253;
            let t255 = t28 * t28;
            let t256 = f64x8::splat(1.0) / t255;
            let t257 = t14 * t256;
            let t259 = f64x8::splat(1.0) / t15 * t2;
            let t260 = t4 * t7;
            let t261 = t260 * t250;
            let t262 = t259 * t261;
            let t264 = t5 * t251;
            let t266 = ((t12).sqrt());
            let t267 = t266 * t2;
            let t268 = t267 * t261;
            let t273 = t22 * t6 / t23 / t8;
            let t275 = -f64x8::splat(0.632975) * t262 - f64x8::splat(0.29896666666666666) * t264 - f64x8::splat(0.1023875) * t268 - f64x8::splat(0.08215666666666667) * t273;
            let t276 = f64x8::splat(1.0) / t31;
            let t277 = t275 * t276;
            let t278 = t257 * t277;
            let t279 = f64x8::splat(1.0) * t278;
            let t280 = t36 * t35;
            let t281 = t280 * t40;
            let t282 = t281 * t89;
            let t283 = f64x8::splat(4.0) * t282;
            let t284 = t39 * t8;
            let t285 = f64x8::splat(1.0) / t284;
            let t286 = t37 * t285;
            let t287 = t286 * t89;
            let t288 = f64x8::splat(4.0) * t287;
            let t289 = f64x8::splat(1.0) / t38;
            let t290 = t35 * t289;
            let t291 = t42 - t290;
            let t294 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t291));
            let t295 = -t291;
            let t298 = ((t52).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t295));
            let t299 = t294 + t298;
            let t300 = t299 * t60;
            let t301 = t300 * t88;
            let t302 = t41 * t301;
            let t306 = t68 * t68;
            let t307 = f64x8::splat(1.0) / t306;
            let t308 = t63 * t307;
            let t313 = -f64x8::splat(1.176575) * t262 - f64x8::splat(0.516475) * t264 - f64x8::splat(0.2103875) * t268 - f64x8::splat(0.104195) * t273;
            let t314 = f64x8::splat(1.0) / t71;
            let t315 = t313 * t314;
            let t321 = t81 * t81;
            let t322 = f64x8::splat(1.0) / t321;
            let t323 = t76 * t322;
            let t328 = -f64x8::splat(0.8630833333333333) * t262 - f64x8::splat(0.301925) * t264 - f64x8::splat(0.05501625) * t268 - f64x8::splat(0.082785) * t273;
            let t329 = f64x8::splat(1.0) / t84;
            let t330 = t328 * t329;
            let t333 = f64x8::splat(0.0005323764196666666) * t5 * t251 * t72 + f64x8::splat(1.0) * t308 * t315 - t254 - t279 + f64x8::splat(0.00018311447306006544) * t5 * t251 * t85 + f64x8::splat(0.5848223622634646) * t323 * t330;
            let t334 = t61 * t333;
            let t335 = t41 * t334;
            let t336 = t300 * t86;
            let t337 = f64x8::splat(0.0197516734986138) * t336;
            let t338 = t61 * t2;
            let t340 = t260 * t250 * t85;
            let t341 = t338 * t340;
            let t342 = f64x8::splat(0.00018311447306006544) * t341;
            let t343 = t61 * t76;
            let t345 = t322 * t328 * t329;
            let t346 = t343 * t345;
            let t347 = f64x8::splat(0.5848223622634646) * t346;
            let t348 = t105 * t145;
            let t349 = f64x8::splat(1.0) / t48;
            let t352 = ((t45).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t349 * t291));
            let t353 = f64x8::splat(1.0) / t53;
            let t356 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t353 * t295));
            let t358 = t352 / f64x8::splat(2.0) + t356 / f64x8::splat(2.0);
            let t360 = t97 * t348 * t358;
            let t361 = f64x8::splat(3.0) * t360;
            let t363 = f64x8::splat(1.0) / t139 / t138;
            let t364 = t38 * t8;
            let t366 = f64x8::splat(1.0) / t23 / t364;
            let t367 = t366 * t111;
            let t369 = t121 * t124;
            let t370 = t57 * t130;
            let t371 = t369 * t370;
            let t373 = f64x8::splat(0.002743937159556463) * t367 * t113 * t371;
            let t374 = t110 * t110;
            let t375 = f64x8::splat(1.0) / t374;
            let t376 = t108 * t375;
            let t377 = t376 * t122;
            let t378 = t124 * t366;
            let t381 = f64x8::splat(0.004878720269691391) * t377 * t378 * t370;
            let t382 = t112 * t113;
            let t383 = t120 * t120;
            let t384 = f64x8::splat(1.0) / t383;
            let t385 = t384 * t124;
            let t387 = t382 * t385 * t128;
            let t388 = t370 * t20;
            let t389 = t132 * t6;
            let t391 = (t254 + t279 + t283 - t288 + t302 + t335 + t337 - t342 - t347) * t113;
            let t393 = t105 * t105;
            let t394 = f64x8::splat(1.0) / t393;
            let t395 = t95 * t394;
            let t396 = t395 * t358;
            let t399 = f64x8::splat(3.0) * t115 * t396 - t117 * t391;
            let t400 = t399 * t119;
            let t402 = t388 * t389 * t400;
            let t406 = f64x8::splat(1.0) / t9 / t364;
            let t407 = t406 * t57;
            let t411 = f64x8::splat(0.0640252003896508) * t126 * t407 * t130 * t134;
            let t413 = t382 * t369 * t128;
            let t414 = t57 * t116;
            let t415 = t414 * t20;
            let t417 = t415 * t389 * t358;
            let t420 = -t373 + t381 - f64x8::splat(0.027439371595564633) * t387 * t402 - t411 - f64x8::splat(0.054878743191129266) * t413 * t417;
            let t421 = t363 * t420;
            let t427 = f64x8::splat(0.25) * t421 * t120 + f64x8::splat(1.0) * t141 * t399 * t119;
            let t429 = f64x8::splat(1.0) / t144;
            let t431 = t97 * t106 * t427 * t429;
            let t432 = v_rho0 * v_rho0;
            let t434 = f64x8::splat(1.0) / t149 / t432;
            let t435 = v_tau0 * t434;
            let t438 = t291 / f64x8::splat(2.0);
            let t439 = t155 * t438;
            let t442 = -t438;
            let t443 = t165 * t442;
            let t446 = t378 / f64x8::splat(3.0);
            let t448 = (-f64x8::splat(5.0) / f64x8::splat(3.0) * t435 * t156 + f64x8::splat(5.0) / f64x8::splat(3.0) * t152 * t439 + f64x8::splat(5.0) / f64x8::splat(3.0) * t162 * t443 + t446) * t173;
            let t450 = t178 * t178;
            let t451 = f64x8::splat(1.0) / t450;
            let t452 = t177 * t451;
            let t454 = f64x8::splat(5.0) / f64x8::splat(3.0) * t439 + f64x8::splat(5.0) / f64x8::splat(3.0) * t443;
            let t455 = t452 * t454;
            let t458 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t174 * t455 + f64x8::splat(5.0) / f64x8::splat(9.0) * t448 * t180;
            let t459 = ((t189).select(t458, f64x8::splat(0.0)));
            let t462 = t191 * t191;
            let t463 = f64x8::splat(1.0) / t462;
            let t464 = t190 * t463;
            let t467 = -f64x8::splat(0.64) * t459 * t192 - f64x8::splat(0.64) * t464 * t459;
            let t468 = t467 * t195;
            let t469 = ((t188).select(f64x8::splat(0.0), t468));
            let t470 = t204 * t204;
            let t471 = f64x8::splat(1.0) / t470;
            let t472 = ((t202).select(f64x8::splat(0.0), t458));
            let t476 = ((t202).select(f64x8::splat(0.0), -f64x8::splat(1.05) * t471 * t472 * t207));
            let t477 = ((t183).select(t469, t476));
            let t478 = t477 * t247;
            let t479 = t213 * t213;
            let t480 = f64x8::splat(1.0) / t479;
            let t483 = -f64x8::splat(0.007408333333333334) * t262 - f64x8::splat(0.01046175) * t264;
            let t484 = t480 * t483;
            let t486 = t217 * t228;
            let t490 = f64x8::splat(1.0) / t226 / t225;
            let t491 = t218 * t490;
            let t492 = t491 * t173;
            let t493 = t177 * t220;
            let t497 = -f64x8::splat(1.0) * t484 * t486 - f64x8::splat(0.014225094736250906) * t492 * t493 * t378;
            let t498 = f64x8::splat(1.0) / t230;
            let t501 = f64x8::splat(0.0285764) * t484 + f64x8::splat(0.0285764) * t497 * t498;
            let t502 = t501 * t237;
            let t503 = t502 * t245;
            let t504 = t233 * t58;
            let t505 = t300 * t245;
            let t508 = t239 * t280;
            let t509 = t508 * t243;
            let t510 = t241 * t284;
            let t511 = f64x8::splat(1.0) / t510;
            let t512 = t240 * t511;
            let t514 = -f64x8::splat(12.0) * t509 + f64x8::splat(12.0) * t512;
            let t516 = t503 - f64x8::splat(2.363) * t504 * t505 + t238 * t514 - t254 - t279 - t283 + t288 - t302 - t335 - t337 + t342 + t347 - t361 - t431;
            let t517 = t210 * t516;
            let t518 = t254 + t279 + t283 - t288 + t302 + t335 + t337 - t342 - t347 + t361 + t431 + t478 + t517;
            let tvrho0 = t518 * t8 + t147 + t248 - t34 + t90 + t92;
            acc_vrho_0 = tvrho0;
            let t520 = -t42 - t290;
            let t523 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t520));
            let t524 = -t520;
            let t527 = ((t52).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t53 * t524));
            let t528 = t523 + t527;
            let t529 = t528 * t60;
            let t530 = t529 * t88;
            let t531 = t41 * t530;
            let t532 = t529 * t86;
            let t533 = f64x8::splat(0.0197516734986138) * t532;
            let t536 = ((t45).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t349 * t520));
            let t539 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t353 * t524));
            let t541 = t536 / f64x8::splat(2.0) + t539 / f64x8::splat(2.0);
            let t543 = t97 * t348 * t541;
            let t544 = f64x8::splat(3.0) * t543;
            let t546 = (t254 + t279 - t283 - t288 + t531 + t335 + t533 - t342 - t347) * t113;
            let t548 = t395 * t541;
            let t551 = f64x8::splat(3.0) * t115 * t548 - t117 * t546;
            let t552 = t551 * t119;
            let t554 = t388 * t389 * t552;
            let t558 = t415 * t389 * t541;
            let t561 = -t373 + t381 - f64x8::splat(0.027439371595564633) * t387 * t554 - t411 - f64x8::splat(0.054878743191129266) * t413 * t558;
            let t562 = t363 * t561;
            let t565 = t141 * t551;
            let t568 = f64x8::splat(0.25) * t562 * t120 + f64x8::splat(1.0) * t565 * t119;
            let t571 = t97 * t106 * t568 * t429;
            let t572 = t520 / f64x8::splat(2.0);
            let t573 = t155 * t572;
            let t576 = v_rho1 * v_rho1;
            let t578 = f64x8::splat(1.0) / t159 / t576;
            let t579 = v_tau1 * t578;
            let t582 = -t572;
            let t583 = t165 * t582;
            let t587 = (f64x8::splat(5.0) / f64x8::splat(3.0) * t152 * t573 - f64x8::splat(5.0) / f64x8::splat(3.0) * t579 * t166 + f64x8::splat(5.0) / f64x8::splat(3.0) * t162 * t583 + t446) * t173;
            let t590 = f64x8::splat(5.0) / f64x8::splat(3.0) * t573 + f64x8::splat(5.0) / f64x8::splat(3.0) * t583;
            let t591 = t452 * t590;
            let t594 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t174 * t591 + f64x8::splat(5.0) / f64x8::splat(9.0) * t587 * t180;
            let t595 = ((t189).select(t594, f64x8::splat(0.0)));
            let t600 = -f64x8::splat(0.64) * t595 * t192 - f64x8::splat(0.64) * t464 * t595;
            let t601 = t600 * t195;
            let t602 = ((t188).select(f64x8::splat(0.0), t601));
            let t603 = ((t202).select(f64x8::splat(0.0), t594));
            let t607 = ((t202).select(f64x8::splat(0.0), -f64x8::splat(1.05) * t471 * t603 * t207));
            let t608 = ((t183).select(t602, t607));
            let t609 = t608 * t247;
            let t610 = t529 * t245;
            let t614 = f64x8::splat(12.0) * t509 + f64x8::splat(12.0) * t512;
            let t616 = t503 - f64x8::splat(2.363) * t504 * t610 + t238 * t614 - t254 - t279 + t283 + t288 - t531 - t335 - t533 + t342 + t347 - t544 - t571;
            let t617 = t210 * t616;
            let t618 = t254 + t279 - t283 - t288 + t531 + t335 + t533 - t342 - t347 + t544 + t571 + t609 + t617;
            let tvrho1 = t618 * t8 + t147 + t248 - t34 + t90 + t92;
            acc_vrho_1 = tvrho1;
            let t620 = t104 * t363;
            let t621 = t112 * t128;
            let t622 = t620 * t621;
            let t623 = t57 * t20;
            let t624 = t389 * t429;
            let t625 = t623 * t624;
            let t626 = t622 * t625;
            let t627 = f64x8::splat(0.0006950474021161377) * t626;
            let t628 = t169 * t173;
            let t629 = t628 * t180;
            let t630 = f64x8::splat(5.0) / f64x8::splat(72.0) * t629;
            let t631 = ((t189).select(-t630, f64x8::splat(0.0)));
            let t636 = -f64x8::splat(0.64) * t631 * t192 - f64x8::splat(0.64) * t464 * t631;
            let t637 = t636 * t195;
            let t638 = ((t188).select(f64x8::splat(0.0), t637));
            let t639 = ((t202).select(f64x8::splat(0.0), -t630));
            let t643 = ((t202).select(f64x8::splat(0.0), -f64x8::splat(1.05) * t471 * t639 * t207));
            let t644 = ((t183).select(t638, t643));
            let t645 = t644 * t247;
            let t646 = t491 * t219;
            let t647 = t220 * t169;
            let t648 = t498 * t237;
            let t649 = t648 * t245;
            let t651 = t646 * t647 * t649;
            let t653 = f64x8::splat(0.00015243824895787514) * t651 - t627;
            let t654 = t210 * t653;
            let tvsigma0 = t8 * (t627 + t645 + t654);
            acc_vsigma_0 = tvsigma0;
            let t656 = f64x8::splat(0.0013900948042322753) * t626;
            let t657 = f64x8::splat(5.0) / f64x8::splat(36.0) * t629;
            let t658 = ((t189).select(-t657, f64x8::splat(0.0)));
            let t663 = -f64x8::splat(0.64) * t658 * t192 - f64x8::splat(0.64) * t464 * t658;
            let t664 = t663 * t195;
            let t665 = ((t188).select(f64x8::splat(0.0), t664));
            let t666 = ((t202).select(f64x8::splat(0.0), -t657));
            let t670 = ((t202).select(f64x8::splat(0.0), -f64x8::splat(1.05) * t471 * t666 * t207));
            let t671 = ((t183).select(t665, t670));
            let t672 = t671 * t247;
            let t674 = f64x8::splat(0.0003048764979157503) * t651 - t656;
            let t675 = t210 * t674;
            let tvsigma1 = t8 * (t656 + t672 + t675);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t677 = t151 * t156;
            let t678 = t219 * t179;
            let t680 = f64x8::splat(5.0) / f64x8::splat(9.0) * t677 * t678;
            let t681 = ((t189).select(t680, f64x8::splat(0.0)));
            let t686 = -f64x8::splat(0.64) * t681 * t192 - f64x8::splat(0.64) * t464 * t681;
            let t687 = t686 * t195;
            let t688 = ((t188).select(f64x8::splat(0.0), t687));
            let t689 = ((t202).select(f64x8::splat(0.0), t680));
            let t693 = ((t202).select(f64x8::splat(0.0), -f64x8::splat(1.05) * t471 * t689 * t207));
            let t694 = ((t183).select(t688, t693));
            let t695 = t8 * t694;
            let tvtau0 = t695 * t247;
            acc_vtau_0 = tvtau0;
            let t696 = t161 * t166;
            let t698 = f64x8::splat(5.0) / f64x8::splat(9.0) * t696 * t678;
            let t699 = ((t189).select(t698, f64x8::splat(0.0)));
            let t704 = -f64x8::splat(0.64) * t699 * t192 - f64x8::splat(0.64) * t464 * t699;
            let t705 = t704 * t195;
            let t706 = ((t188).select(f64x8::splat(0.0), t705));
            let t707 = ((t202).select(f64x8::splat(0.0), t698));
            let t711 = ((t202).select(f64x8::splat(0.0), -f64x8::splat(1.05) * t471 * t707 * t207));
            let t712 = ((t183).select(t706, t711));
            let t713 = t8 * t712;
            let tvtau1 = t713 * t247;
            acc_vtau_1 = tvtau1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
