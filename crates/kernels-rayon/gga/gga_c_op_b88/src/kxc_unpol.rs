//! GGA_C_OP_B88 kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_b88.c`
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
pub fn gga_c_op_b88_kxc_unpol(
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
            let t1 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = (t1) | ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold));
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t6 = -t5;
            let t7 = ((t1).select(t5, (t1).select(t6, f64x8::splat(0.0))));
            let t8 = t7 * t7;
            let t9 = f64x8::splat(1.0) - t8;
            let t10 = t9 * v_rho;
            let t11 = f64x8::splat(1.0) + t7;
            let t14 = (t11 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t15 = f64x8::splat(M_CBRT3);
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t20 = t16 / t18;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = t20 * t21;
            let t23 = f64x8::splat(M_CBRT2);
            let t24 = (t11).simd_le(zeta_threshold);
            let t25 = f64x8::splat(1.0) - t7;
            let t26 = (t25).simd_le(zeta_threshold);
            let t27 = ((t24).select(t5, (t26).select(t6, t7)));
            let t28 = f64x8::splat(1.0) + t27;
            let t29 = t28 * v_rho;
            let t30 = (simd::cbrt(t29));
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t23 * t31;
            let t33 = t23 * t23;
            let t34 = v_sigma * t33;
            let t35 = v_rho * v_rho;
            let t36 = (simd::cbrt(v_rho));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t40 = ((v_sigma).sqrt());
            let t41 = t40 * t23;
            let t43 = f64x8::splat(1.0) / t36 / v_rho;
            let t45 = (simd::ln(t41 * t43 + ((((t41 * t43) * (t41 * t43)) + f64x8::splat(1.0)).sqrt())));
            let t46 = t43 * t45;
            let t49 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t41 * t46;
            let t50 = f64x8::splat(1.0) / t49;
            let t55 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t22 * t34 * t39 * t50;
            let t56 = f64x8::splat(1.0) / t55;
            let t60 = ((t14).select(f64x8::splat(0.0), t22 * t32 * t56 / f64x8::splat(9.0)));
            let t64 = (t25 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t65 = ((t26).select(t5, (t24).select(t6, -t7)));
            let t66 = f64x8::splat(1.0) + t65;
            let t67 = t66 * v_rho;
            let t68 = (simd::cbrt(t67));
            let t69 = f64x8::splat(1.0) / t68;
            let t70 = t23 * t69;
            let t74 = ((t64).select(f64x8::splat(0.0), t22 * t70 * t56 / f64x8::splat(9.0)));
            let t75 = t60 + t74;
            let t76 = (t75).simd_eq(f64x8::splat(0.0));
            let t77 = ((t76).select(f64x8::splat(f64::EPSILON), t75));
            let t80 = f64x8::splat(3.6011538) / t77 + f64x8::splat(0.5764);
            let t81 = t77 * t77;
            let t82 = t81 * t81;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t81 * t77;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = f64x8::splat(1.0) / t81;
            let t90 = f64x8::splat(31.390124030721) * t83 + f64x8::splat(14.9643497914092) * t86 + f64x8::splat(1.7833359087) * t88;
            let t91 = f64x8::splat(1.0) / t90;
            let tzk0 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t10 * t80 * t91));
            acc_zk = tzk0;
            let t95 = t9 * t80;
            let t99 = f64x8::splat(1.0) / t30 / t29;
            let t105 = t55 * t55;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t35 * v_rho;
            let t109 = f64x8::splat(1.0) / t37 / t107;
            let t114 = t21 * v_sigma;
            let t115 = t20 * t114;
            let t116 = t33 * t39;
            let t117 = t49 * t49;
            let t118 = f64x8::splat(1.0) / t117;
            let t121 = f64x8::splat(1.0) / t36 / t35 * t45;
            let t125 = t34 * t39 + f64x8::splat(1.0);
            let t126 = ((t125).sqrt());
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t109 * t127;
            let t131 = -f64x8::splat(0.0336) * t41 * t121 - f64x8::splat(0.0336) * t34 * t128;
            let t132 = t118 * t131;
            let t133 = t116 * t132;
            let t136 = -f64x8::splat(0.002488888888888889) * t22 * t34 * t109 * t50 - f64x8::splat(0.0009333333333333333) * t115 * t133;
            let t137 = t106 * t136;
            let t142 = ((t14).select(f64x8::splat(0.0), -t22 * t23 * t99 * t56 * t28 / f64x8::splat(27.0) - t22 * t32 * t137 / f64x8::splat(9.0)));
            let t144 = f64x8::splat(1.0) / t68 / t67;
            let t154 = ((t64).select(f64x8::splat(0.0), -t22 * t23 * t144 * t56 * t66 / f64x8::splat(27.0) - t22 * t70 * t137 / f64x8::splat(9.0)));
            let t156 = ((t76).select(f64x8::splat(0.0), t142 + t154));
            let t161 = t90 * t90;
            let t162 = f64x8::splat(1.0) / t161;
            let t163 = t80 * t162;
            let t165 = f64x8::splat(1.0) / t82 / t77;
            let t166 = t165 * t156;
            let t168 = t83 * t156;
            let t172 = -f64x8::splat(125.560496122884) * t166 - f64x8::splat(44.8930493742276) * t168 - f64x8::splat(3.5666718174) * t86 * t156;
            let t177 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t95 * t91 + f64x8::splat(0.90028845) * t10 * t88 * t156 * t91 + f64x8::splat(0.25) * t10 * t163 * t172));
            let tvrho0 = v_rho * t177 + tzk0;
            acc_vrho = tvrho0;
            let t183 = f64x8::splat(1.0) / t40 * t23;
            let t188 = f64x8::splat(0.0126) * t183 * t46 + f64x8::splat(0.0126) * t116 * t127;
            let t189 = t118 * t188;
            let t190 = t116 * t189;
            let t193 = f64x8::splat(0.0009333333333333333) * t22 * t116 * t50 - f64x8::splat(0.0009333333333333333) * t115 * t190;
            let t194 = t106 * t193;
            let t198 = ((t14).select(f64x8::splat(0.0), -t22 * t32 * t194 / f64x8::splat(9.0)));
            let t202 = ((t64).select(f64x8::splat(0.0), -t22 * t70 * t194 / f64x8::splat(9.0)));
            let t204 = ((t76).select(f64x8::splat(0.0), t198 + t202));
            let t209 = t165 * t204;
            let t211 = t83 * t204;
            let t213 = t86 * t204;
            let t215 = -f64x8::splat(125.560496122884) * t209 - f64x8::splat(44.8930493742276) * t211 - f64x8::splat(3.5666718174) * t213;
            let t220 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.90028845) * t10 * t88 * t204 * t91 + f64x8::splat(0.25) * t10 * t163 * t215));
            let tvsigma0 = v_rho * t220;
            acc_vsigma = tvsigma0;
            let t222 = t9 * t88;
            let t223 = t156 * t91;
            let t229 = t156 * t156;
            let t234 = t28 * t28;
            let t237 = f64x8::splat(1.0) / t30 / t234 / t35;
            let t244 = t20 * t21 * t23;
            let t245 = t99 * t106;
            let t246 = t28 * t136;
            let t251 = f64x8::splat(1.0) / t105 / t55;
            let t252 = t136 * t136;
            let t253 = t251 * t252;
            let t257 = t35 * t35;
            let t259 = f64x8::splat(1.0) / t37 / t257;
            let t264 = t33 * t109;
            let t265 = t264 * t132;
            let t269 = f64x8::splat(1.0) / t117 / t49;
            let t270 = t131 * t131;
            let t271 = t269 * t270;
            let t272 = t116 * t271;
            let t277 = f64x8::splat(1.0) / t36 / t107 * t45;
            let t280 = t259 * t127;
            let t283 = v_sigma * v_sigma;
            let t284 = t283 * t23;
            let t287 = f64x8::splat(1.0) / t36 / t257 / t107;
            let t289 = f64x8::splat(1.0) / t126 / t125;
            let t293 = f64x8::splat(0.0784) * t41 * t277 + f64x8::splat(0.168) * t34 * t280 - f64x8::splat(0.0896) * t284 * t287 * t289;
            let t294 = t118 * t293;
            let t295 = t116 * t294;
            let t298 = f64x8::splat(0.009125925925925926) * t22 * t34 * t259 * t50 + f64x8::splat(0.004977777777777778) * t115 * t265 + f64x8::splat(0.0018666666666666666) * t115 * t272 - f64x8::splat(0.0009333333333333333) * t115 * t295;
            let t299 = t106 * t298;
            let t304 = ((t14).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(81.0) * t22 * t23 * t237 * t56 * t234 + f64x8::splat(2.0) / f64x8::splat(27.0) * t244 * t245 * t246 + f64x8::splat(2.0) / f64x8::splat(9.0) * t22 * t32 * t253 - t22 * t32 * t299 / f64x8::splat(9.0)));
            let t305 = t66 * t66;
            let t308 = f64x8::splat(1.0) / t68 / t305 / t35;
            let t314 = t144 * t106;
            let t315 = t66 * t136;
            let t326 = ((t64).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(81.0) * t22 * t23 * t308 * t56 * t305 + f64x8::splat(2.0) / f64x8::splat(27.0) * t244 * t314 * t315 + f64x8::splat(2.0) / f64x8::splat(9.0) * t22 * t70 * t253 - t22 * t70 * t299 / f64x8::splat(9.0)));
            let t328 = ((t76).select(f64x8::splat(0.0), t304 + t326));
            let t333 = t10 * t88;
            let t334 = t156 * t162;
            let t335 = t334 * t172;
            let t339 = f64x8::splat(1.0) / t161 / t90;
            let t340 = t80 * t339;
            let t341 = t172 * t172;
            let t346 = f64x8::splat(1.0) / t82 / t81;
            let t347 = t346 * t229;
            let t351 = t165 * t229;
            let t359 = f64x8::splat(627.80248061442) * t347 - f64x8::splat(125.560496122884) * t165 * t328 + f64x8::splat(179.5721974969104) * t351 - f64x8::splat(44.8930493742276) * t83 * t328 + f64x8::splat(10.7000154522) * t83 * t229 - f64x8::splat(3.5666718174) * t86 * t328;
            let t364 = ((t4).select(f64x8::splat(0.0), f64x8::splat(1.8005769) * t222 * t223 + f64x8::splat(0.5) * t95 * t162 * t172 - f64x8::splat(1.8005769) * t10 * t86 * t229 * t91 + f64x8::splat(0.90028845) * t10 * t88 * t328 * t91 - f64x8::splat(1.8005769) * t333 * t335 - f64x8::splat(0.5) * t10 * t340 * t341 + f64x8::splat(0.25) * t10 * t163 * t359));
            let tv2rho20 = v_rho * t364 + f64x8::splat(2.0) * t177;
            acc_v2rho2 = tv2rho20;
            let t366 = t204 * t91;
            let t369 = t10 * t86;
            let t370 = t366 * t156;
            let t373 = t193 * t28;
            let t377 = t31 * t251;
            let t378 = t193 * t136;
            let t387 = t264 * t189;
            let t390 = t269 * t188;
            let t391 = t390 * t131;
            let t399 = t257 * t35;
            let t401 = f64x8::splat(1.0) / t36 / t399;
            let t402 = t23 * t401;
            let t403 = t289 * v_sigma;
            let t406 = -f64x8::splat(0.0168) * t183 * t121 - f64x8::splat(0.0504) * t264 * t127 + f64x8::splat(0.0336) * t402 * t403;
            let t407 = t118 * t406;
            let t408 = t116 * t407;
            let t411 = -f64x8::splat(0.002488888888888889) * t22 * t264 * t50 - f64x8::splat(0.0009333333333333333) * t22 * t133 + f64x8::splat(0.002488888888888889) * t115 * t387 + f64x8::splat(0.0018666666666666666) * t115 * t116 * t391 - f64x8::splat(0.0009333333333333333) * t115 * t408;
            let t412 = t106 * t411;
            let t417 = ((t14).select(f64x8::splat(0.0), t244 * t245 * t373 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t244 * t377 * t378 - t22 * t32 * t412 / f64x8::splat(9.0)));
            let t418 = t193 * t66;
            let t422 = t69 * t251;
            let t430 = ((t64).select(f64x8::splat(0.0), t244 * t314 * t418 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t244 * t422 * t378 - t22 * t70 * t412 / f64x8::splat(9.0)));
            let t432 = ((t76).select(f64x8::splat(0.0), t417 + t430));
            let t437 = t204 * t162;
            let t438 = t437 * t172;
            let t444 = t334 * t215;
            let t447 = t10 * t80;
            let t448 = t339 * t215;
            let t449 = t448 * t172;
            let t452 = t346 * t204;
            let t455 = t165 * t432;
            let t459 = t83 * t432;
            let t465 = f64x8::splat(627.80248061442) * t452 * t156 - f64x8::splat(125.560496122884) * t455 + f64x8::splat(179.5721974969104) * t209 * t156 - f64x8::splat(44.8930493742276) * t459 + f64x8::splat(10.7000154522) * t211 * t156 - f64x8::splat(3.5666718174) * t86 * t432;
            let t470 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.90028845) * t222 * t366 - f64x8::splat(1.8005769) * t369 * t370 + f64x8::splat(0.90028845) * t10 * t88 * t432 * t91 - f64x8::splat(0.90028845) * t333 * t438 + f64x8::splat(0.25) * t95 * t162 * t215 - f64x8::splat(0.90028845) * t333 * t444 - f64x8::splat(0.5) * t447 * t449 + f64x8::splat(0.25) * t10 * t163 * t465));
            let tv2rhosigma0 = v_rho * t470 + t220;
            acc_v2rhosigma = tv2rhosigma0;
            let t472 = t204 * t204;
            let t477 = t193 * t193;
            let t478 = t251 * t477;
            let t484 = t188 * t188;
            let t485 = t269 * t484;
            let t486 = t116 * t485;
            let t491 = f64x8::splat(1.0) / t40 / v_sigma * t23;
            let t494 = f64x8::splat(1.0) / v_sigma;
            let t495 = t494 * t33;
            let t496 = t39 * t127;
            let t499 = t257 * v_rho;
            let t501 = f64x8::splat(1.0) / t36 / t499;
            let t505 = -f64x8::splat(0.0063) * t491 * t46 + f64x8::splat(0.0063) * t495 * t496 - f64x8::splat(0.0126) * t23 * t501 * t289;
            let t506 = t118 * t505;
            let t507 = t116 * t506;
            let t510 = -f64x8::splat(0.0018666666666666666) * t22 * t190 + f64x8::splat(0.0018666666666666666) * t115 * t486 - f64x8::splat(0.0009333333333333333) * t115 * t507;
            let t511 = t106 * t510;
            let t516 = ((t14).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(9.0) * t22 * t32 * t478 - t22 * t32 * t511 / f64x8::splat(9.0)));
            let t524 = ((t64).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(9.0) * t22 * t70 * t478 - t22 * t70 * t511 / f64x8::splat(9.0)));
            let t526 = ((t76).select(f64x8::splat(0.0), t516 + t524));
            let t531 = t437 * t215;
            let t534 = t215 * t215;
            let t538 = t346 * t472;
            let t540 = t165 * t526;
            let t542 = t165 * t472;
            let t544 = t83 * t526;
            let t550 = f64x8::splat(627.80248061442) * t538 - f64x8::splat(125.560496122884) * t540 + f64x8::splat(179.5721974969104) * t542 - f64x8::splat(44.8930493742276) * t544 + f64x8::splat(10.7000154522) * t83 * t472 - f64x8::splat(3.5666718174) * t86 * t526;
            let t555 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(1.8005769) * t10 * t86 * t472 * t91 + f64x8::splat(0.90028845) * t10 * t88 * t526 * t91 - f64x8::splat(1.8005769) * t333 * t531 - f64x8::splat(0.5) * t10 * t340 * t534 + f64x8::splat(0.25) * t10 * t163 * t550));
            let tv2sigma20 = v_rho * t555;
            acc_v2sigma2 = tv2sigma20;
            let t558 = f64x8::splat(1.0) / t82 / t85;
            let t559 = t229 * t156;
            let t562 = t346 * t156;
            let t565 = t234 * t28;
            let t568 = f64x8::splat(1.0) / t30 / t565 / t107;
            let t574 = t237 * t106;
            let t579 = t99 * t251;
            let t588 = t105 * t105;
            let t589 = f64x8::splat(1.0) / t588;
            let t590 = t252 * t136;
            let t591 = t589 * t590;
            let t595 = t136 * t298;
            let t600 = f64x8::splat(1.0) / t37 / t499;
            let t605 = t33 * t259;
            let t606 = t605 * t132;
            let t609 = t264 * t271;
            let t612 = t264 * t294;
            let t615 = t117 * t117;
            let t616 = f64x8::splat(1.0) / t615;
            let t617 = t270 * t131;
            let t618 = t616 * t617;
            let t619 = t116 * t618;
            let t622 = t269 * t131;
            let t623 = t622 * t293;
            let t629 = f64x8::splat(1.0) / t36 / t257 * t45;
            let t635 = t257 * t257;
            let t637 = f64x8::splat(1.0) / t36 / t635;
            let t641 = t283 * v_sigma;
            let t642 = t635 * t107;
            let t643 = f64x8::splat(1.0) / t642;
            let t645 = t125 * t125;
            let t647 = f64x8::splat(1.0) / t126 / t645;
            let t650 = -f64x8::splat(0.2613333333333333) * t41 * t629 - f64x8::splat(0.8885333333333333) * t34 * t600 * t127 + f64x8::splat(1.1050666666666666) * t284 * t637 * t289 - f64x8::splat(0.7168) * t641 * t643 * t647;
            let t651 = t118 * t650;
            let t652 = t116 * t651;
            let t655 = -f64x8::splat(0.042587654320987656) * t22 * t34 * t600 * t50 - f64x8::splat(0.02737777777777778) * t115 * t606 - f64x8::splat(0.014933333333333333) * t115 * t609 + f64x8::splat(0.007466666666666667) * t115 * t612 - f64x8::splat(0.0056) * t115 * t619 + f64x8::splat(0.0056) * t115 * t116 * t623 - f64x8::splat(0.0009333333333333333) * t115 * t652;
            let t656 = t106 * t655;
            let t661 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(28.0) / f64x8::splat(243.0) * t22 * t23 * t568 * t56 * t565 - f64x8::splat(4.0) / f64x8::splat(27.0) * t244 * t574 * t234 * t136 - f64x8::splat(2.0) / f64x8::splat(9.0) * t244 * t579 * t28 * t252 + t244 * t245 * t28 * t298 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t22 * t32 * t591 + f64x8::splat(2.0) / f64x8::splat(3.0) * t244 * t377 * t595 - t22 * t32 * t656 / f64x8::splat(9.0)));
            let t662 = t305 * t66;
            let t665 = f64x8::splat(1.0) / t68 / t662 / t107;
            let t671 = t308 * t106;
            let t676 = t144 * t251;
            let t695 = ((t64).select(f64x8::splat(0.0), -f64x8::splat(28.0) / f64x8::splat(243.0) * t22 * t23 * t665 * t56 * t662 - f64x8::splat(4.0) / f64x8::splat(27.0) * t244 * t671 * t305 * t136 - f64x8::splat(2.0) / f64x8::splat(9.0) * t244 * t676 * t66 * t252 + t244 * t314 * t66 * t298 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t22 * t70 * t591 + f64x8::splat(2.0) / f64x8::splat(3.0) * t244 * t422 * t595 - t22 * t70 * t656 / f64x8::splat(9.0)));
            let t697 = ((t76).select(f64x8::splat(0.0), t661 + t695));
            let t712 = -f64x8::splat(3766.81488368652) * t558 * t559 + f64x8::splat(1883.40744184326) * t562 * t328 - f64x8::splat(125.560496122884) * t165 * t697 - f64x8::splat(897.860987484552) * t346 * t559 + f64x8::splat(538.7165924907312) * t166 * t328 - f64x8::splat(44.8930493742276) * t83 * t697 - f64x8::splat(42.8000618088) * t165 * t559 + f64x8::splat(32.1000463566) * t168 * t328 - f64x8::splat(3.5666718174) * t86 * t697;
            let t717 = t339 * t172 * t359;
            let t720 = t161 * t161;
            let t721 = f64x8::splat(1.0) / t720;
            let t722 = t80 * t721;
            let t723 = t341 * t172;
            let t727 = t334 * t359;
            let t734 = t156 * t339;
            let t735 = t734 * t341;
            let t749 = t9 * t86;
            let t750 = t229 * t91;
            let t753 = t328 * t162;
            let t754 = t753 * t172;
            let t757 = t223 * t328;
            let t760 = t229 * t162;
            let t761 = t760 * t172;
            let t768 = f64x8::splat(0.25) * t10 * t163 * t712 - f64x8::splat(1.5) * t447 * t717 + f64x8::splat(1.5) * t10 * t722 * t723 - f64x8::splat(2.70086535) * t333 * t727 + f64x8::splat(0.90028845) * t10 * t88 * t697 * t91 + f64x8::splat(5.4017307) * t333 * t735 + f64x8::splat(0.75) * t95 * t162 * t359 + f64x8::splat(2.70086535) * t222 * t328 * t91 - f64x8::splat(1.5) * t95 * t339 * t341 - f64x8::splat(5.4017307) * t222 * t335 - f64x8::splat(5.4017307) * t749 * t750 - f64x8::splat(2.70086535) * t333 * t754 - f64x8::splat(5.4017307) * t369 * t757 + f64x8::splat(5.4017307) * t369 * t761 + f64x8::splat(5.4017307) * t10 * t83 * t559 * t91;
            let t769 = ((t4).select(f64x8::splat(0.0), t768));
            let tv3rho30 = v_rho * t769 + f64x8::splat(3.0) * t364;
            acc_v3rho3 = tv3rho30;
            let t772 = t558 * t204;
            let t775 = t346 * t432;
            let t780 = t193 * t234;
            let t788 = t411 * t28;
            let t792 = t31 * t589;
            let t793 = t193 * t252;
            let t797 = t411 * t136;
            let t801 = t193 * t298;
            let t814 = t605 * t189;
            let t820 = t264 * t407;
            let t824 = t616 * t188 * t270;
            let t828 = t269 * t406;
            let t829 = t828 * t131;
            let t833 = t390 * t293;
            let t841 = t23 * t287;
            let t844 = t635 * t35;
            let t846 = f64x8::splat(1.0) / t844 * t647;
            let t849 = f64x8::splat(0.0392) * t183 * t277 + f64x8::splat(0.2072) * t605 * t127 - f64x8::splat(0.3472) * t841 * t403 + f64x8::splat(0.2688) * t846 * t283;
            let t850 = t118 * t849;
            let t851 = t116 * t850;
            let t854 = f64x8::splat(0.009125925925925926) * t22 * t605 * t50 + f64x8::splat(0.004977777777777778) * t22 * t265 + f64x8::splat(0.0018666666666666666) * t22 * t272 - f64x8::splat(0.0009333333333333333) * t22 * t295 - f64x8::splat(0.009125925925925926) * t115 * t814 - f64x8::splat(0.009955555555555556) * t115 * t264 * t391 + f64x8::splat(0.004977777777777778) * t115 * t820 - f64x8::splat(0.0056) * t115 * t116 * t824 + f64x8::splat(0.0037333333333333333) * t115 * t116 * t829 + f64x8::splat(0.0018666666666666666) * t115 * t116 * t833 - f64x8::splat(0.0009333333333333333) * t115 * t851;
            let t855 = t106 * t854;
            let t860 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(4.0) / f64x8::splat(81.0) * t244 * t574 * t780 - f64x8::splat(4.0) / f64x8::splat(27.0) * t244 * t579 * t373 * t136 + f64x8::splat(2.0) / f64x8::splat(27.0) * t244 * t245 * t788 - f64x8::splat(2.0) / f64x8::splat(3.0) * t244 * t792 * t793 + f64x8::splat(4.0) / f64x8::splat(9.0) * t244 * t377 * t797 + f64x8::splat(2.0) / f64x8::splat(9.0) * t244 * t377 * t801 - t22 * t32 * t855 / f64x8::splat(9.0)));
            let t861 = t193 * t305;
            let t869 = t411 * t66;
            let t873 = t69 * t589;
            let t887 = ((t64).select(f64x8::splat(0.0), -f64x8::splat(4.0) / f64x8::splat(81.0) * t244 * t671 * t861 - f64x8::splat(4.0) / f64x8::splat(27.0) * t244 * t676 * t418 * t136 + f64x8::splat(2.0) / f64x8::splat(27.0) * t244 * t314 * t869 - f64x8::splat(2.0) / f64x8::splat(3.0) * t244 * t873 * t793 + f64x8::splat(4.0) / f64x8::splat(9.0) * t244 * t422 * t797 + f64x8::splat(2.0) / f64x8::splat(9.0) * t244 * t422 * t801 - t22 * t70 * t855 / f64x8::splat(9.0)));
            let t889 = ((t76).select(f64x8::splat(0.0), t860 + t887));
            let t890 = t165 * t889;
            let t898 = t83 * t889;
            let t908 = -f64x8::splat(3766.81488368652) * t772 * t229 + f64x8::splat(1255.60496122884) * t775 * t156 + f64x8::splat(627.80248061442) * t452 * t328 - f64x8::splat(125.560496122884) * t890 - f64x8::splat(897.860987484552) * t452 * t229 + f64x8::splat(359.1443949938208) * t455 * t156 + f64x8::splat(179.5721974969104) * t209 * t328 - f64x8::splat(44.8930493742276) * t898 - f64x8::splat(42.8000618088) * t209 * t229 + f64x8::splat(21.4000309044) * t459 * t156 + f64x8::splat(10.7000154522) * t211 * t328 - f64x8::splat(3.5666718174) * t86 * t889;
            let t912 = t721 * t215;
            let t913 = t912 * t341;
            let t918 = t215 * t172;
            let t919 = t734 * t918;
            let t922 = t760 * t215;
            let t925 = t204 * t339;
            let t926 = t925 * t341;
            let t929 = t448 * t359;
            let t932 = t339 * t465;
            let t933 = t932 * t172;
            let t936 = t753 * t215;
            let t939 = t334 * t465;
            let t942 = t437 * t359;
            let t945 = f64x8::splat(0.25) * t10 * t163 * t908 + f64x8::splat(1.5) * t447 * t913 - f64x8::splat(1.8005769) * t222 * t444 + f64x8::splat(3.6011538) * t333 * t919 + f64x8::splat(1.8005769) * t369 * t922 + f64x8::splat(1.8005769) * t333 * t926 - f64x8::splat(0.5) * t447 * t929 - f64x8::splat(1.0) * t447 * t933 - f64x8::splat(0.90028845) * t333 * t936 - f64x8::splat(1.8005769) * t333 * t939 - f64x8::splat(0.90028845) * t333 * t942;
            let t946 = t432 * t162;
            let t947 = t946 * t172;
            let t955 = t432 * t91;
            let t958 = t156 * t172;
            let t962 = t10 * t83;
            let t963 = t366 * t229;
            let t974 = t955 * t156;
            let t977 = t366 * t328;
            let t980 = -f64x8::splat(1.8005769) * t333 * t947 - f64x8::splat(1.0) * t95 * t449 + f64x8::splat(0.5) * t95 * t162 * t465 + f64x8::splat(1.8005769) * t222 * t955 + f64x8::splat(3.6011538) * t369 * t437 * t958 + f64x8::splat(5.4017307) * t962 * t963 + f64x8::splat(0.90028845) * t10 * t88 * t889 * t91 - f64x8::splat(1.8005769) * t222 * t438 - f64x8::splat(3.6011538) * t749 * t370 - f64x8::splat(3.6011538) * t369 * t974 - f64x8::splat(1.8005769) * t369 * t977;
            let t982 = ((t4).select(f64x8::splat(0.0), t945 + t980));
            let tv3rho2sigma0 = v_rho * t982 + f64x8::splat(2.0) * t470;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t984 = t472 * t91;
            let t987 = t984 * t156;
            let t990 = t366 * t432;
            let t993 = t472 * t162;
            let t994 = t993 * t172;
            let t997 = t526 * t91;
            let t1000 = t997 * t156;
            let t1003 = t477 * t28;
            let t1007 = t477 * t136;
            let t1011 = t193 * t411;
            let t1015 = t510 * t28;
            let t1019 = t510 * t136;
            let t1026 = t20 * t21 * t33;
            let t1027 = t39 * t269;
            let t1028 = t188 * t131;
            let t1034 = t264 * t485;
            let t1037 = t616 * t484;
            let t1038 = t1037 * t131;
            let t1042 = t390 * t406;
            let t1046 = t264 * t506;
            let t1049 = t269 * t505;
            let t1050 = t1049 * t131;
            let t1060 = t635 * v_rho;
            let t1062 = f64x8::splat(1.0) / t1060 * t647;
            let t1065 = f64x8::splat(0.0084) * t491 * t121 - f64x8::splat(0.0084) * t495 * t128 + f64x8::splat(0.084) * t402 * t289 - f64x8::splat(0.1008) * t1062 * v_sigma;
            let t1066 = t118 * t1065;
            let t1067 = t116 * t1066;
            let t1070 = f64x8::splat(0.004977777777777778) * t22 * t387 + f64x8::splat(0.0037333333333333333) * t1026 * t1027 * t1028 - f64x8::splat(0.0018666666666666666) * t22 * t408 - f64x8::splat(0.004977777777777778) * t115 * t1034 - f64x8::splat(0.0056) * t115 * t116 * t1038 + f64x8::splat(0.0037333333333333333) * t115 * t116 * t1042 + f64x8::splat(0.002488888888888889) * t115 * t1046 + f64x8::splat(0.0018666666666666666) * t115 * t116 * t1050 - f64x8::splat(0.0009333333333333333) * t115 * t1067;
            let t1071 = t106 * t1070;
            let t1076 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(27.0) * t244 * t579 * t1003 - f64x8::splat(2.0) / f64x8::splat(3.0) * t244 * t792 * t1007 + f64x8::splat(4.0) / f64x8::splat(9.0) * t244 * t377 * t1011 + t244 * t245 * t1015 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t244 * t377 * t1019 - t22 * t32 * t1071 / f64x8::splat(9.0)));
            let t1077 = t477 * t66;
            let t1087 = t510 * t66;
            let t1098 = ((t64).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(27.0) * t244 * t676 * t1077 - f64x8::splat(2.0) / f64x8::splat(3.0) * t244 * t873 * t1007 + f64x8::splat(4.0) / f64x8::splat(9.0) * t244 * t422 * t1011 + t244 * t314 * t1087 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t244 * t422 * t1019 - t22 * t70 * t1071 / f64x8::splat(9.0)));
            let t1100 = ((t76).select(f64x8::splat(0.0), t1076 + t1098));
            let t1105 = t526 * t162;
            let t1106 = t1105 * t172;
            let t1111 = t215 * t156;
            let t1116 = t946 * t215;
            let t1122 = t437 * t465;
            let t1128 = t734 * t534;
            let t1131 = t721 * t534;
            let t1132 = t1131 * t172;
            let t1135 = t448 * t465;
            let t1141 = t334 * t550;
            let t1144 = t339 * t550;
            let t1145 = t1144 * t172;
            let t1148 = t558 * t472;
            let t1153 = t346 * t526;
            let t1156 = t165 * t1100;
            let t1164 = t83 * t1100;
            let t1174 = -f64x8::splat(3766.81488368652) * t1148 * t156 + f64x8::splat(1255.60496122884) * t452 * t432 + f64x8::splat(627.80248061442) * t1153 * t156 - f64x8::splat(125.560496122884) * t1156 - f64x8::splat(897.860987484552) * t538 * t156 + f64x8::splat(359.1443949938208) * t209 * t432 + f64x8::splat(179.5721974969104) * t540 * t156 - f64x8::splat(44.8930493742276) * t1164 - f64x8::splat(42.8000618088) * t542 * t156 + f64x8::splat(21.4000309044) * t211 * t432 + f64x8::splat(10.7000154522) * t544 * t156 - f64x8::splat(3.5666718174) * t86 * t1100;
            let t1178 = -f64x8::splat(1.8005769) * t333 * t1116 + f64x8::splat(3.6011538) * t333 * t925 * t918 - f64x8::splat(1.8005769) * t333 * t1122 - f64x8::splat(0.5) * t95 * t339 * t534 + f64x8::splat(1.8005769) * t333 * t1128 + f64x8::splat(1.5) * t447 * t1132 - f64x8::splat(1.0) * t447 * t1135 + f64x8::splat(0.25) * t95 * t162 * t550 - f64x8::splat(0.90028845) * t333 * t1141 - f64x8::splat(0.5) * t447 * t1145 + f64x8::splat(0.25) * t10 * t163 * t1174;
            let t1180 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(1.8005769) * t749 * t984 + f64x8::splat(5.4017307) * t962 * t987 - f64x8::splat(3.6011538) * t369 * t990 + f64x8::splat(1.8005769) * t369 * t994 + f64x8::splat(0.90028845) * t222 * t997 - f64x8::splat(1.8005769) * t369 * t1000 + f64x8::splat(0.90028845) * t10 * t88 * t1100 * t91 - f64x8::splat(0.90028845) * t333 * t1106 - f64x8::splat(1.8005769) * t222 * t531 + f64x8::splat(3.6011538) * t369 * t437 * t1111 + t1178));
            let tv3rhosigma20 = v_rho * t1180 + t555;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t1182 = t472 * t204;
            let t1187 = t366 * t526;
            let t1190 = t993 * t215;
            let t1193 = t477 * t193;
            let t1194 = t589 * t1193;
            let t1198 = t193 * t510;
            let t1206 = t484 * t188;
            let t1207 = t616 * t1206;
            let t1208 = t116 * t1207;
            let t1211 = t390 * t505;
            let t1217 = f64x8::splat(1.0) / t40 / t283 * t23;
            let t1220 = f64x8::splat(1.0) / t283;
            let t1221 = t1220 * t33;
            let t1224 = t494 * t23;
            let t1225 = t501 * t289;
            let t1228 = f64x8::splat(1.0) / t635;
            let t1231 = f64x8::splat(0.00945) * t1217 * t46 - f64x8::splat(0.00945) * t1221 * t496 - f64x8::splat(0.0063) * t1224 * t1225 + f64x8::splat(0.0378) * t1228 * t647;
            let t1232 = t118 * t1231;
            let t1233 = t116 * t1232;
            let t1236 = f64x8::splat(0.0056) * t22 * t486 - f64x8::splat(0.0028) * t22 * t507 - f64x8::splat(0.0056) * t115 * t1208 + f64x8::splat(0.0056) * t115 * t116 * t1211 - f64x8::splat(0.0009333333333333333) * t115 * t1233;
            let t1237 = t106 * t1236;
            let t1242 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t22 * t32 * t1194 + f64x8::splat(2.0) / f64x8::splat(3.0) * t244 * t377 * t1198 - t22 * t32 * t1237 / f64x8::splat(9.0)));
            let t1253 = ((t64).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t22 * t70 * t1194 + f64x8::splat(2.0) / f64x8::splat(3.0) * t244 * t422 * t1198 - t22 * t70 * t1237 / f64x8::splat(9.0)));
            let t1255 = ((t76).select(f64x8::splat(0.0), t1242 + t1253));
            let t1260 = t1105 * t215;
            let t1263 = t925 * t534;
            let t1266 = t437 * t550;
            let t1269 = t534 * t215;
            let t1273 = t448 * t550;
            let t1276 = t558 * t1182;
            let t1280 = t165 * t1255;
            let t1282 = t346 * t1182;
            let t1286 = t83 * t1255;
            let t1294 = -f64x8::splat(3766.81488368652) * t1276 + f64x8::splat(1883.40744184326) * t452 * t526 - f64x8::splat(125.560496122884) * t1280 - f64x8::splat(897.860987484552) * t1282 + f64x8::splat(538.7165924907312) * t209 * t526 - f64x8::splat(44.8930493742276) * t1286 - f64x8::splat(42.8000618088) * t165 * t1182 + f64x8::splat(32.1000463566) * t211 * t526 - f64x8::splat(3.5666718174) * t86 * t1255;
            let t1299 = ((t4).select(f64x8::splat(0.0), f64x8::splat(5.4017307) * t10 * t83 * t1182 * t91 - f64x8::splat(5.4017307) * t369 * t1187 + f64x8::splat(5.4017307) * t369 * t1190 + f64x8::splat(0.90028845) * t10 * t88 * t1255 * t91 - f64x8::splat(2.70086535) * t333 * t1260 + f64x8::splat(5.4017307) * t333 * t1263 - f64x8::splat(2.70086535) * t333 * t1266 + f64x8::splat(1.5) * t10 * t722 * t1269 - f64x8::splat(1.5) * t447 * t1273 + f64x8::splat(0.25) * t10 * t163 * t1294));
            let tv3sigma30 = v_rho * t1299;
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
