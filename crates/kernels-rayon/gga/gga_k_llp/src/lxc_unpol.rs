//! GGA_K_LLP lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_llp.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_llp_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * t21;
            let t23 = t20 * t22;
            let t24 = param_beta * t4;
            let t26 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t27 = f64x8::splat(1.0) / t26;
            let t28 = f64x8::splat(M_CBRT4);
            let t29 = t27 * t28;
            let t30 = t24 * t29;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t22 / t34;
            let t37 = param_gamma * param_beta;
            let t38 = ((v_sigma).sqrt());
            let t39 = t37 * t38;
            let t41 = f64x8::splat(1.0) / t21 / v_rho;
            let t45 = (simd::ln(t38 * t31 * t41 + ((((t38 * t31 * t41) * (t38 * t31 * t41)) + f64x8::splat(1.0)).sqrt())));
            let t46 = t31 * t41 * t45;
            let t48 = f64x8::splat(1.0) + t39 * t46;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t36 * t49;
            let t54 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t50;
            let t58 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t54));
            let tzk0 = f64x8::splat(2.0) * t58;
            acc_zk = tzk0;
            let t60 = t20 / t21;
            let t64 = t34 * v_rho;
            let t66 = f64x8::splat(1.0) / t22 / t64;
            let t67 = t66 * t49;
            let t71 = t48 * t48;
            let t72 = f64x8::splat(1.0) / t71;
            let t73 = t36 * t72;
            let t75 = f64x8::splat(1.0) / t21 / t34;
            let t77 = t31 * t75 * t45;
            let t79 = t37 * v_sigma;
            let t80 = t32 * t66;
            let t82 = t33 * t36 + f64x8::splat(1.0);
            let t83 = ((t82).sqrt());
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = t80 * t84;
            let t88 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t39 * t77 - f64x8::splat(4.0) / f64x8::splat(3.0) * t79 * t85;
            let t93 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t30 * t33 * t67 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t88;
            let t98 = ((t2).select(f64x8::splat(0.0), t7 * t60 * t54 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t93));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t98 + f64x8::splat(2.0) * t58;
            acc_vrho = tvrho0;
            let t101 = t24 * t27;
            let t102 = t28 * t32;
            let t106 = t37 / t38;
            let t108 = t32 * t36;
            let t109 = t108 * t84;
            let t112 = t106 * t46 / f64x8::splat(2.0) + t37 * t109 / f64x8::splat(2.0);
            let t117 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t112 + f64x8::splat(2.0) / f64x8::splat(9.0) * t101 * t102 * t50;
            let t121 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t117));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t121;
            acc_vsigma = tvsigma0;
            let t124 = t20 * t41;
            let t131 = t34 * t34;
            let t133 = f64x8::splat(1.0) / t22 / t131;
            let t134 = t133 * t49;
            let t138 = t66 * t72;
            let t144 = f64x8::splat(1.0) / t71 / t48;
            let t145 = t36 * t144;
            let t146 = t88 * t88;
            let t152 = f64x8::splat(1.0) / t21 / t64;
            let t154 = t31 * t152 * t45;
            let t157 = t32 * t133;
            let t158 = t157 * t84;
            let t161 = v_sigma * v_sigma;
            let t162 = t37 * t161;
            let t165 = f64x8::splat(1.0) / t21 / t131 / t64;
            let t168 = f64x8::splat(1.0) / t83 / t82;
            let t169 = t31 * t165 * t168;
            let t172 = f64x8::splat(28.0) / f64x8::splat(9.0) * t39 * t154 + f64x8::splat(20.0) / f64x8::splat(3.0) * t79 * t158 - f64x8::splat(32.0) / f64x8::splat(9.0) * t162 * t169;
            let t177 = f64x8::splat(176.0) / f64x8::splat(81.0) * t30 * t33 * t134 + f64x8::splat(32.0) / f64x8::splat(27.0) * t30 * t33 * t138 * t88 + f64x8::splat(4.0) / f64x8::splat(9.0) * t30 * t33 * t145 * t146 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t172;
            let t182 = ((t2).select(f64x8::splat(0.0), -t7 * t124 * t54 / f64x8::splat(30.0) + t7 * t60 * t93 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t177));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t182 + f64x8::splat(4.0) * t98;
            acc_v2rho2 = tv2rho20;
            let t191 = t72 * t88;
            let t200 = t24 * t29 * v_sigma;
            let t201 = t144 * t112;
            let t202 = t201 * t88;
            let t203 = t108 * t202;
            let t210 = t37 * t31;
            let t211 = t131 * t34;
            let t213 = f64x8::splat(1.0) / t21 / t211;
            let t218 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t106 * t77 - f64x8::splat(2.0) * t37 * t85 + f64x8::splat(4.0) / f64x8::splat(3.0) * t210 * t213 * t168 * v_sigma;
            let t223 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t101 * t102 * t67 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t108 * t191 + f64x8::splat(16.0) / f64x8::splat(27.0) * t30 * t33 * t138 * t112 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200 * t203 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t218;
            let t228 = ((t2).select(f64x8::splat(0.0), t7 * t60 * t117 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t223));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t228 + f64x8::splat(2.0) * t121;
            acc_v2rhosigma = tv2rhosigma0;
            let t231 = t72 * t112;
            let t235 = t112 * t112;
            let t242 = t37 / t38 / v_sigma;
            let t245 = f64x8::splat(1.0) / v_sigma;
            let t246 = t37 * t245;
            let t249 = t131 * v_rho;
            let t252 = t31 / t21 / t249;
            let t253 = t252 * t168;
            let t256 = -t242 * t46 / f64x8::splat(4.0) + t246 * t109 / f64x8::splat(4.0) - t37 * t253 / f64x8::splat(2.0);
            let t261 = -f64x8::splat(4.0) / f64x8::splat(9.0) * t30 * t108 * t231 + f64x8::splat(4.0) / f64x8::splat(9.0) * t30 * t33 * t145 * t235 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t256;
            let t265 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t261));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t265;
            acc_v2sigma2 = tv2sigma20;
            let t268 = t20 * t75;
            let t279 = f64x8::splat(1.0) / t22 / t249;
            let t280 = t279 * t49;
            let t284 = t133 * t72;
            let t289 = t66 * t144;
            let t298 = t71 * t71;
            let t299 = f64x8::splat(1.0) / t298;
            let t300 = t36 * t299;
            let t301 = t146 * t88;
            let t306 = t144 * t88;
            let t307 = t306 * t172;
            let t308 = t108 * t307;
            let t314 = t31 / t21 / t131 * t45;
            let t318 = t32 * t279 * t84;
            let t321 = t131 * t131;
            let t323 = f64x8::splat(1.0) / t21 / t321;
            let t328 = t161 * v_sigma;
            let t329 = t321 * t64;
            let t330 = f64x8::splat(1.0) / t329;
            let t332 = t82 * t82;
            let t334 = f64x8::splat(1.0) / t83 / t332;
            let t338 = -f64x8::splat(280.0) / f64x8::splat(27.0) * t39 * t314 - f64x8::splat(952.0) / f64x8::splat(27.0) * t79 * t318 + f64x8::splat(1184.0) / f64x8::splat(27.0) * t162 * t31 * t323 * t168 - f64x8::splat(256.0) / f64x8::splat(9.0) * t37 * t328 * t330 * t334;
            let t343 = -f64x8::splat(2464.0) / f64x8::splat(243.0) * t30 * t33 * t280 - f64x8::splat(176.0) / f64x8::splat(27.0) * t30 * t33 * t284 * t88 - f64x8::splat(32.0) / f64x8::splat(9.0) * t30 * t33 * t289 * t146 + f64x8::splat(16.0) / f64x8::splat(9.0) * t30 * t33 * t138 * t172 - f64x8::splat(4.0) / f64x8::splat(3.0) * t30 * t33 * t300 * t301 + f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t308 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t338;
            let t348 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(45.0) * t7 * t268 * t54 - t7 * t124 * t93 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t7 * t60 * t177 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t343));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t348 + f64x8::splat(6.0) * t182;
            acc_v3rho3 = tv3rho30;
            let t364 = t144 * t146;
            let t368 = t72 * t172;
            let t376 = t80 * t202;
            let t384 = t299 * t112 * t146;
            let t385 = t108 * t384;
            let t388 = t144 * t218;
            let t389 = t388 * t88;
            let t390 = t108 * t389;
            let t393 = t201 * t172;
            let t394 = t108 * t393;
            let t405 = t321 * t34;
            let t407 = f64x8::splat(1.0) / t405 * t334;
            let t411 = f64x8::splat(14.0) / f64x8::splat(9.0) * t106 * t154 + f64x8::splat(74.0) / f64x8::splat(9.0) * t37 * t158 - f64x8::splat(124.0) / f64x8::splat(9.0) * t210 * t165 * t168 * v_sigma + f64x8::splat(32.0) / f64x8::splat(3.0) * t37 * t407 * t161;
            let t416 = f64x8::splat(176.0) / f64x8::splat(81.0) * t101 * t102 * t134 + f64x8::splat(32.0) / f64x8::splat(27.0) * t30 * t80 * t191 + f64x8::splat(4.0) / f64x8::splat(9.0) * t30 * t108 * t364 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t108 * t368 - f64x8::splat(176.0) / f64x8::splat(81.0) * t30 * t33 * t284 * t112 - f64x8::splat(64.0) / f64x8::splat(27.0) * t200 * t376 + f64x8::splat(32.0) / f64x8::splat(27.0) * t30 * t33 * t138 * t218 - f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t385 + f64x8::splat(8.0) / f64x8::splat(9.0) * t200 * t390 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200 * t394 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t411;
            let t421 = ((t2).select(f64x8::splat(0.0), -t7 * t124 * t117 / f64x8::splat(30.0) + t7 * t60 * t223 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t416));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t421 + f64x8::splat(4.0) * t228;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t432 = t72 * t218;
            let t440 = t299 * t235;
            let t441 = t440 * t88;
            let t442 = t108 * t441;
            let t445 = t201 * t218;
            let t446 = t108 * t445;
            let t453 = t144 * t256;
            let t454 = t453 * t88;
            let t455 = t108 * t454;
            let t463 = t31 * t213 * t168;
            let t466 = t321 * v_rho;
            let t468 = f64x8::splat(1.0) / t466 * t334;
            let t472 = t242 * t77 / f64x8::splat(3.0) - t246 * t85 / f64x8::splat(3.0) + f64x8::splat(10.0) / f64x8::splat(3.0) * t37 * t463 - f64x8::splat(4.0) * t37 * t468 * v_sigma;
            let t477 = f64x8::splat(32.0) / f64x8::splat(27.0) * t30 * t80 * t231 + f64x8::splat(8.0) / f64x8::splat(9.0) * t30 * t203 - f64x8::splat(4.0) / f64x8::splat(9.0) * t30 * t108 * t432 - f64x8::splat(32.0) / f64x8::splat(27.0) * t30 * t33 * t289 * t235 - f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t442 + f64x8::splat(8.0) / f64x8::splat(9.0) * t200 * t446 + f64x8::splat(16.0) / f64x8::splat(27.0) * t30 * t33 * t138 * t256 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200 * t455 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t472;
            let t482 = ((t2).select(f64x8::splat(0.0), t7 * t60 * t261 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t477));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t482 + f64x8::splat(2.0) * t265;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t485 = t144 * t235;
            let t489 = t72 * t256;
            let t493 = t235 * t112;
            let t498 = t201 * t256;
            let t499 = t108 * t498;
            let t504 = t37 / t38 / t161;
            let t508 = t37 / t161;
            let t513 = f64x8::splat(1.0) / t321;
            let t517 = f64x8::splat(3.0) / f64x8::splat(8.0) * t504 * t46 - f64x8::splat(3.0) / f64x8::splat(8.0) * t508 * t109 - t246 * t253 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t37 * t513 * t334;
            let t522 = f64x8::splat(4.0) / f64x8::splat(3.0) * t30 * t108 * t485 - f64x8::splat(2.0) / f64x8::splat(3.0) * t30 * t108 * t489 - f64x8::splat(4.0) / f64x8::splat(3.0) * t30 * t33 * t300 * t493 + f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t499 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t517;
            let t526 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t522));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t526;
            acc_v3sigma3 = tv3sigma30;
            let t543 = f64x8::splat(1.0) / t22 / t211;
            let t548 = t279 * t72;
            let t553 = t133 * t144;
            let t562 = t66 * t299;
            let t575 = f64x8::splat(1.0) / t298 / t48;
            let t576 = t36 * t575;
            let t577 = t146 * t146;
            let t587 = t172 * t172;
            let t609 = t321 * t131;
            let t615 = t161 * t161;
            let t622 = f64x8::splat(1.0) / t83 / t332 / t82;
            let t632 = f64x8::splat(41888.0) / f64x8::splat(729.0) * t30 * t33 * t543 * t49 + f64x8::splat(9856.0) / f64x8::splat(243.0) * t30 * t33 * t548 * t88 + f64x8::splat(704.0) / f64x8::splat(27.0) * t30 * t33 * t553 * t146 - f64x8::splat(352.0) / f64x8::splat(27.0) * t30 * t33 * t284 * t172 + f64x8::splat(128.0) / f64x8::splat(9.0) * t30 * t33 * t562 * t301 - f64x8::splat(128.0) / f64x8::splat(9.0) * t200 * t80 * t307 + f64x8::splat(64.0) / f64x8::splat(27.0) * t30 * t33 * t138 * t338 + f64x8::splat(16.0) / f64x8::splat(3.0) * t30 * t33 * t576 * t577 - f64x8::splat(8.0) * t200 * t108 * t299 * t146 * t172 + f64x8::splat(4.0) / f64x8::splat(3.0) * t30 * t33 * t145 * t587 + f64x8::splat(16.0) / f64x8::splat(9.0) * t200 * t108 * t306 * t338 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * (f64x8::splat(3640.0) / f64x8::splat(81.0) * t39 * t252 * t45 + f64x8::splat(5768.0) / f64x8::splat(27.0) * t79 * t32 * t543 * t84 - f64x8::splat(37216.0) / f64x8::splat(81.0) * t162 * t31 / t21 / t466 * t168 + f64x8::splat(17920.0) / f64x8::splat(27.0) * t37 * t328 / t609 * t334 - f64x8::splat(5120.0) / f64x8::splat(27.0) * t37 * t615 / t22 / t321 / t211 * t622 * t32);
            let t637 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(14.0) / f64x8::splat(135.0) * t7 * t20 * t152 * t54 + f64x8::splat(8.0) / f64x8::splat(45.0) * t7 * t268 * t93 - t7 * t124 * t177 / f64x8::splat(5.0) + f64x8::splat(2.0) / f64x8::splat(5.0) * t7 * t60 * t343 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t632));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t637 + f64x8::splat(8.0) * t348;
            acc_v4rho4 = tv4rho40;
            let t745 = t108 * t299;
            let t746 = t112 * t88;
            let t751 = f64x8::splat(352.0) / f64x8::splat(27.0) * t200 * t157 * t202 - f64x8::splat(64.0) / f64x8::splat(9.0) * t200 * t80 * t389 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * (-f64x8::splat(140.0) / f64x8::splat(27.0) * t106 * t314 - f64x8::splat(364.0) / f64x8::splat(9.0) * t37 * t318 + f64x8::splat(3320.0) / f64x8::splat(27.0) * t210 * t323 * t168 * v_sigma - f64x8::splat(1952.0) / f64x8::splat(9.0) * t37 * t330 * t334 * t161 + f64x8::splat(640.0) / f64x8::splat(9.0) * t37 / t22 / t321 / t249 * t622 * t328 * t32) - f64x8::splat(4.0) / f64x8::splat(3.0) * t30 * t108 * t299 * t301 + f64x8::splat(4.0) / f64x8::splat(3.0) * t30 * t308 - f64x8::splat(176.0) / f64x8::splat(27.0) * t30 * t33 * t284 * t218 + f64x8::splat(16.0) / f64x8::splat(9.0) * t30 * t33 * t138 * t411 - f64x8::splat(32.0) / f64x8::splat(9.0) * t30 * t80 * t364 + f64x8::splat(32.0) / f64x8::splat(3.0) * t200 * t80 * t384 + f64x8::splat(16.0) / f64x8::splat(3.0) * t200 * t108 * t575 * t112 * t301 - f64x8::splat(4.0) * t200 * t745 * t746 * t172;
            let t757 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(45.0) * t7 * t268 * t117 - t7 * t124 * t223 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t7 * t60 * t416 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * (-f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t108 * t72 * t338 - f64x8::splat(2464.0) / f64x8::splat(243.0) * t101 * t102 * t280 - f64x8::splat(176.0) / f64x8::splat(27.0) * t30 * t157 * t191 + f64x8::splat(16.0) / f64x8::splat(9.0) * t30 * t80 * t368 + f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t108 * t144 * t411 * t88 + f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t108 * t388 * t172 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200 * t108 * t201 * t338 - f64x8::splat(32.0) / f64x8::splat(9.0) * t200 * t80 * t393 - f64x8::splat(4.0) * t200 * t108 * t299 * t218 * t146 + f64x8::splat(2464.0) / f64x8::splat(243.0) * t30 * t33 * t548 * t112 + t751)));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t757 + f64x8::splat(6.0) * t421;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t822 = t218 * t218;
            let t827 = f64x8::splat(32.0) / f64x8::splat(27.0) * t30 * t33 * t138 * t472 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * (-f64x8::splat(7.0) / f64x8::splat(9.0) * t242 * t154 + f64x8::splat(7.0) / f64x8::splat(9.0) * t246 * t158 - f64x8::splat(22.0) * t37 * t169 + f64x8::splat(188.0) / f64x8::splat(3.0) * t37 * t407 * v_sigma - f64x8::splat(80.0) / f64x8::splat(3.0) * t37 / t22 / t609 * t622 * t161 * t32) - f64x8::splat(176.0) / f64x8::splat(81.0) * t30 * t33 * t284 * t256 - f64x8::splat(64.0) / f64x8::splat(27.0) * t200 * t80 * t454 + f64x8::splat(8.0) / f64x8::splat(9.0) * t200 * t108 * t144 * t472 * t88 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200 * t108 * t453 * t172 + f64x8::splat(352.0) / f64x8::splat(81.0) * t30 * t33 * t553 * t235 + f64x8::splat(64.0) / f64x8::splat(9.0) * t200 * t80 * t441 - f64x8::splat(128.0) / f64x8::splat(27.0) * t200 * t80 * t445 - f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t108 * t440 * t172 + f64x8::splat(8.0) / f64x8::splat(9.0) * t30 * t33 * t145 * t822;
            let t864 = f64x8::splat(8.0) / f64x8::splat(9.0) * t200 * t108 * t201 * t411 - f64x8::splat(8.0) / f64x8::splat(3.0) * t30 * t385 - f64x8::splat(4.0) / f64x8::splat(9.0) * t30 * t108 * t72 * t411 + f64x8::splat(64.0) / f64x8::splat(27.0) * t30 * t80 * t432 - f64x8::splat(352.0) / f64x8::splat(81.0) * t30 * t157 * t231 - f64x8::splat(128.0) / f64x8::splat(27.0) * t30 * t376 + f64x8::splat(16.0) / f64x8::splat(9.0) * t30 * t390 + f64x8::splat(8.0) / f64x8::splat(9.0) * t30 * t394 - f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t108 * t299 * t256 * t146 + f64x8::splat(16.0) / f64x8::splat(3.0) * t200 * t108 * t575 * t235 * t146 - f64x8::splat(16.0) / f64x8::splat(3.0) * t200 * t745 * t746 * t218;
            let t870 = ((t2).select(f64x8::splat(0.0), -t7 * t124 * t261 / f64x8::splat(30.0) + t7 * t60 * t477 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * (t827 + t864)));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t870 + f64x8::splat(4.0) * t482;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t950 = -f64x8::splat(32.0) / f64x8::splat(9.0) * t30 * t80 * t485 - f64x8::splat(4.0) * t30 * t442 + f64x8::splat(8.0) / f64x8::splat(3.0) * t30 * t446 + f64x8::splat(16.0) / f64x8::splat(9.0) * t30 * t80 * t489 + f64x8::splat(4.0) / f64x8::splat(3.0) * t30 * t455 - f64x8::splat(2.0) / f64x8::splat(3.0) * t30 * t108 * t72 * t472 + f64x8::splat(32.0) / f64x8::splat(9.0) * t30 * t33 * t562 * t493 + f64x8::splat(16.0) / f64x8::splat(3.0) * t200 * t108 * t575 * t493 * t88 - f64x8::splat(4.0) * t200 * t108 * t440 * t218 - f64x8::splat(32.0) / f64x8::splat(9.0) * t200 * t80 * t498 - f64x8::splat(4.0) * t200 * t745 * t112 * t256 * t88 + f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t108 * t388 * t256 + f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t108 * t201 * t472 + f64x8::splat(16.0) / f64x8::splat(27.0) * t30 * t33 * t138 * t517 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200 * t108 * t144 * t517 * t88 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * (-t504 * t77 / f64x8::splat(2.0) + t508 * t85 / f64x8::splat(2.0) + t246 * t463 / f64x8::splat(3.0) - f64x8::splat(14.0) * t37 * t468 + f64x8::splat(10.0) * t37 / t22 / t329 * t622 * v_sigma * t32);
            let t955 = ((t2).select(f64x8::splat(0.0), t7 * t60 * t522 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t950));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t955 + f64x8::splat(2.0) * t526;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t968 = t235 * t235;
            let t977 = t256 * t256;
            let t1016 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * (-f64x8::splat(16.0) / f64x8::splat(3.0) * t30 * t108 * t299 * t493 + f64x8::splat(16.0) / f64x8::splat(3.0) * t30 * t499 - f64x8::splat(8.0) / f64x8::splat(9.0) * t30 * t108 * t72 * t517 + f64x8::splat(16.0) / f64x8::splat(3.0) * t30 * t33 * t576 * t968 - f64x8::splat(8.0) * t200 * t108 * t440 * t256 + f64x8::splat(4.0) / f64x8::splat(3.0) * t30 * t33 * t145 * t977 + f64x8::splat(16.0) / f64x8::splat(9.0) * t200 * t108 * t201 * t517 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * (-f64x8::splat(15.0) / f64x8::splat(16.0) * t37 / t38 / t328 * t46 + f64x8::splat(15.0) / f64x8::splat(16.0) * t37 / t328 * t109 + f64x8::splat(5.0) / f64x8::splat(8.0) * t508 * t253 + f64x8::splat(3.0) / f64x8::splat(4.0) * t37 * t245 * t513 * t334 - f64x8::splat(15.0) / f64x8::splat(4.0) * t37 / t22 / t405 * t622 * t32))));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t1016;
            acc_v4sigma4 = tv4sigma40;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho3sigma.into(); v4rho3sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho2sigma2.into(); v4rho2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rhosigma3.into(); v4rhosigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4sigma4.into(); v4sigma4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
