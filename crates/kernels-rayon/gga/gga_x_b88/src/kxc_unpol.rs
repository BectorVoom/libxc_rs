//! GGA_X_B88 kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b88.c`
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
pub fn gga_x_b88_kxc_unpol(
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
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = t3 * t3;
            let t21 = param_beta * t20;
            let t23 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t24 * t25;
            let t27 = t21 * t26;
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t28 * t28;
            let t30 = v_sigma * t29;
            let t31 = v_rho * v_rho;
            let t32 = t18 * t18;
            let t34 = f64x8::splat(1.0) / t32 / t31;
            let t35 = param_gamma * param_beta;
            let t36 = ((v_sigma).sqrt());
            let t37 = t35 * t36;
            let t39 = f64x8::splat(1.0) / t18 / v_rho;
            let t43 = (simd::ln(t36 * t28 * t39 + ((((t36 * t28 * t39) * (t36 * t28 * t39)) + f64x8::splat(1.0)).sqrt())));
            let t44 = t28 * t39 * t43;
            let t46 = t37 * t44 + f64x8::splat(1.0);
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t34 * t47;
            let t52 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t48;
            let t56 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t52));
            let tzk0 = f64x8::splat(2.0) * t56;
            acc_zk = tzk0;
            let t58 = t17 / t32;
            let t62 = t31 * v_rho;
            let t64 = f64x8::splat(1.0) / t32 / t62;
            let t65 = t64 * t47;
            let t69 = t46 * t46;
            let t70 = f64x8::splat(1.0) / t69;
            let t71 = t34 * t70;
            let t75 = t28 / t18 / t31 * t43;
            let t77 = t35 * v_sigma;
            let t78 = t29 * t64;
            let t80 = t30 * t34 + f64x8::splat(1.0);
            let t81 = ((t80).sqrt());
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t78 * t82;
            let t86 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t37 * t75 - f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t83;
            let t91 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t27 * t30 * t65 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t86;
            let t96 = ((t2).select(f64x8::splat(0.0), -t6 * t58 * t52 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t91));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t96 + f64x8::splat(2.0) * t56;
            acc_vrho = tvrho0;
            let t99 = t21 * t24;
            let t100 = t25 * t29;
            let t104 = t35 / t36;
            let t106 = t29 * t34;
            let t107 = t106 * t82;
            let t110 = t104 * t44 / f64x8::splat(2.0) + t35 * t107 / f64x8::splat(2.0);
            let t115 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t110 + f64x8::splat(2.0) / f64x8::splat(9.0) * t99 * t100 * t48;
            let t119 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t115));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t119;
            acc_vsigma = tvsigma0;
            let t124 = t17 / t32 / v_rho;
            let t131 = t31 * t31;
            let t133 = f64x8::splat(1.0) / t32 / t131;
            let t134 = t133 * t47;
            let t138 = t64 * t70;
            let t144 = f64x8::splat(1.0) / t69 / t46;
            let t145 = t34 * t144;
            let t146 = t86 * t86;
            let t154 = t28 / t18 / t62 * t43;
            let t157 = t29 * t133;
            let t158 = t157 * t82;
            let t161 = v_sigma * v_sigma;
            let t162 = t35 * t161;
            let t165 = f64x8::splat(1.0) / t18 / t131 / t62;
            let t168 = f64x8::splat(1.0) / t81 / t80;
            let t169 = t28 * t165 * t168;
            let t172 = f64x8::splat(28.0) / f64x8::splat(9.0) * t37 * t154 + f64x8::splat(20.0) / f64x8::splat(3.0) * t77 * t158 - f64x8::splat(32.0) / f64x8::splat(9.0) * t162 * t169;
            let t177 = f64x8::splat(176.0) / f64x8::splat(81.0) * t27 * t30 * t134 + f64x8::splat(32.0) / f64x8::splat(27.0) * t27 * t30 * t138 * t86 + f64x8::splat(4.0) / f64x8::splat(9.0) * t27 * t30 * t145 * t146 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t172;
            let t182 = ((t2).select(f64x8::splat(0.0), t6 * t124 * t52 / f64x8::splat(12.0) - t6 * t58 * t91 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t177));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t182 + f64x8::splat(4.0) * t96;
            acc_v2rho2 = tv2rho20;
            let t191 = t70 * t86;
            let t200 = t21 * t26 * v_sigma;
            let t201 = t144 * t110;
            let t202 = t201 * t86;
            let t203 = t106 * t202;
            let t210 = t35 * t28;
            let t211 = t131 * t31;
            let t213 = f64x8::splat(1.0) / t18 / t211;
            let t218 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t104 * t75 - f64x8::splat(2.0) * t35 * t83 + f64x8::splat(4.0) / f64x8::splat(3.0) * t210 * t213 * t168 * v_sigma;
            let t223 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t99 * t100 * t65 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t106 * t191 + f64x8::splat(16.0) / f64x8::splat(27.0) * t27 * t30 * t138 * t110 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200 * t203 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t218;
            let t228 = ((t2).select(f64x8::splat(0.0), -t6 * t58 * t115 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t223));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t228 + f64x8::splat(2.0) * t119;
            acc_v2rhosigma = tv2rhosigma0;
            let t231 = t70 * t110;
            let t235 = t110 * t110;
            let t242 = t35 / t36 / v_sigma;
            let t245 = f64x8::splat(1.0) / v_sigma;
            let t246 = t35 * t245;
            let t249 = t131 * v_rho;
            let t252 = t28 / t18 / t249;
            let t253 = t252 * t168;
            let t256 = -t242 * t44 / f64x8::splat(4.0) + t246 * t107 / f64x8::splat(4.0) - t35 * t253 / f64x8::splat(2.0);
            let t261 = -f64x8::splat(4.0) / f64x8::splat(9.0) * t27 * t106 * t231 + f64x8::splat(4.0) / f64x8::splat(9.0) * t27 * t30 * t145 * t235 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t256;
            let t265 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t261));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t265;
            acc_v2sigma2 = tv2sigma20;
            let t268 = t17 * t34;
            let t279 = f64x8::splat(1.0) / t32 / t249;
            let t280 = t279 * t47;
            let t284 = t133 * t70;
            let t289 = t64 * t144;
            let t298 = t69 * t69;
            let t299 = f64x8::splat(1.0) / t298;
            let t300 = t34 * t299;
            let t301 = t146 * t86;
            let t306 = t144 * t86;
            let t307 = t306 * t172;
            let t308 = t106 * t307;
            let t314 = t28 / t18 / t131 * t43;
            let t318 = t29 * t279 * t82;
            let t321 = t131 * t131;
            let t323 = f64x8::splat(1.0) / t18 / t321;
            let t328 = t161 * v_sigma;
            let t329 = t321 * t62;
            let t330 = f64x8::splat(1.0) / t329;
            let t332 = t80 * t80;
            let t334 = f64x8::splat(1.0) / t81 / t332;
            let t338 = -f64x8::splat(280.0) / f64x8::splat(27.0) * t37 * t314 - f64x8::splat(952.0) / f64x8::splat(27.0) * t77 * t318 + f64x8::splat(1184.0) / f64x8::splat(27.0) * t162 * t28 * t323 * t168 - f64x8::splat(256.0) / f64x8::splat(9.0) * t35 * t328 * t330 * t334;
            let t343 = -f64x8::splat(2464.0) / f64x8::splat(243.0) * t27 * t30 * t280 - f64x8::splat(176.0) / f64x8::splat(27.0) * t27 * t30 * t284 * t86 - f64x8::splat(32.0) / f64x8::splat(9.0) * t27 * t30 * t289 * t146 + f64x8::splat(16.0) / f64x8::splat(9.0) * t27 * t30 * t138 * t172 - f64x8::splat(4.0) / f64x8::splat(3.0) * t27 * t30 * t300 * t301 + f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t308 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t338;
            let t348 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t268 * t52 + t6 * t124 * t91 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t58 * t177 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t343));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t348 + f64x8::splat(6.0) * t182;
            acc_v3rho3 = tv3rho30;
            let t364 = t144 * t146;
            let t368 = t70 * t172;
            let t376 = t78 * t202;
            let t384 = t299 * t110 * t146;
            let t385 = t106 * t384;
            let t388 = t144 * t218;
            let t389 = t388 * t86;
            let t390 = t106 * t389;
            let t393 = t201 * t172;
            let t394 = t106 * t393;
            let t405 = t321 * t31;
            let t407 = f64x8::splat(1.0) / t405 * t334;
            let t411 = f64x8::splat(14.0) / f64x8::splat(9.0) * t104 * t154 + f64x8::splat(74.0) / f64x8::splat(9.0) * t35 * t158 - f64x8::splat(124.0) / f64x8::splat(9.0) * t210 * t165 * t168 * v_sigma + f64x8::splat(32.0) / f64x8::splat(3.0) * t35 * t407 * t161;
            let t416 = f64x8::splat(176.0) / f64x8::splat(81.0) * t99 * t100 * t134 + f64x8::splat(32.0) / f64x8::splat(27.0) * t27 * t78 * t191 + f64x8::splat(4.0) / f64x8::splat(9.0) * t27 * t106 * t364 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t106 * t368 - f64x8::splat(176.0) / f64x8::splat(81.0) * t27 * t30 * t284 * t110 - f64x8::splat(64.0) / f64x8::splat(27.0) * t200 * t376 + f64x8::splat(32.0) / f64x8::splat(27.0) * t27 * t30 * t138 * t218 - f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t385 + f64x8::splat(8.0) / f64x8::splat(9.0) * t200 * t390 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200 * t394 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t411;
            let t421 = ((t2).select(f64x8::splat(0.0), t6 * t124 * t115 / f64x8::splat(12.0) - t6 * t58 * t223 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t416));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t421 + f64x8::splat(4.0) * t228;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t432 = t70 * t218;
            let t440 = t299 * t235;
            let t441 = t440 * t86;
            let t442 = t106 * t441;
            let t445 = t201 * t218;
            let t446 = t106 * t445;
            let t453 = t144 * t256;
            let t454 = t453 * t86;
            let t455 = t106 * t454;
            let t463 = t28 * t213 * t168;
            let t466 = t321 * v_rho;
            let t468 = f64x8::splat(1.0) / t466 * t334;
            let t472 = t242 * t75 / f64x8::splat(3.0) - t246 * t83 / f64x8::splat(3.0) + f64x8::splat(10.0) / f64x8::splat(3.0) * t35 * t463 - f64x8::splat(4.0) * t35 * t468 * v_sigma;
            let t477 = f64x8::splat(32.0) / f64x8::splat(27.0) * t27 * t78 * t231 + f64x8::splat(8.0) / f64x8::splat(9.0) * t27 * t203 - f64x8::splat(4.0) / f64x8::splat(9.0) * t27 * t106 * t432 - f64x8::splat(32.0) / f64x8::splat(27.0) * t27 * t30 * t289 * t235 - f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t442 + f64x8::splat(8.0) / f64x8::splat(9.0) * t200 * t446 + f64x8::splat(16.0) / f64x8::splat(27.0) * t27 * t30 * t138 * t256 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200 * t455 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t472;
            let t482 = ((t2).select(f64x8::splat(0.0), -t6 * t58 * t261 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t477));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t482 + f64x8::splat(2.0) * t265;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t485 = t144 * t235;
            let t489 = t70 * t256;
            let t493 = t235 * t110;
            let t498 = t201 * t256;
            let t499 = t106 * t498;
            let t504 = t35 / t36 / t161;
            let t508 = t35 / t161;
            let t513 = f64x8::splat(1.0) / t321;
            let t517 = f64x8::splat(3.0) / f64x8::splat(8.0) * t504 * t44 - f64x8::splat(3.0) / f64x8::splat(8.0) * t508 * t107 - t246 * t253 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t35 * t513 * t334;
            let t522 = f64x8::splat(4.0) / f64x8::splat(3.0) * t27 * t106 * t485 - f64x8::splat(2.0) / f64x8::splat(3.0) * t27 * t106 * t489 - f64x8::splat(4.0) / f64x8::splat(3.0) * t27 * t30 * t300 * t493 + f64x8::splat(4.0) / f64x8::splat(3.0) * t200 * t499 - f64x8::splat(2.0) / f64x8::splat(9.0) * t27 * t30 * t71 * t517;
            let t526 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t522));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t526;
            acc_v3sigma3 = tv3sigma30;
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
        ip += 8;
    }
}
