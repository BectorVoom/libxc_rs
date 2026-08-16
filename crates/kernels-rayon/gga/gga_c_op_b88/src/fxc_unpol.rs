//! GGA_C_OP_B88 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_b88.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_op_b88_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = 1.0 <= zeta_threshold;
        let t4 = t1 || rho[ip] / 2.0 <= dens_threshold;
        let t5 = zeta_threshold - 1.0;
        let t6 = -t5;
        let t7 = piecewise5(t1, t5, t1, t6, 0.0);
        let t8 = t7 * t7;
        let t9 = 1.0 - t8;
        let t10 = t9 * rho[ip];
        let t11 = 1.0 + t7;
        let t14 = t11 * rho[ip] / 2.0 <= dens_threshold;
        let t15 = M_CBRT3;
        let t16 = t15 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t20 = t16 / t18;
        let t21 = M_CBRT4;
        let t22 = t20 * t21;
        let t23 = M_CBRT2;
        let t24 = t11 <= zeta_threshold;
        let t25 = 1.0 - t7;
        let t26 = t25 <= zeta_threshold;
        let t27 = piecewise5(t24, t5, t26, t6, t7);
        let t28 = 1.0 + t27;
        let t29 = t28 * rho[ip];
        let t30 = pow_1_3(t29);
        let t31 = 1.0 / t30;
        let t32 = t23 * t31;
        let t33 = t23 * t23;
        let t34 = sigma[ip] * t33;
        let t35 = rho[ip] * rho[ip];
        let t36 = pow_1_3(rho[ip]);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = f64::sqrt(sigma[ip]);
        let t41 = t40 * t23;
        let t43 = 1.0 / t36 / rho[ip];
        let t45 = f64::ln(t41 * t43 + f64::sqrt(pow_2(t41 * t43) + 1.0));
        let t46 = t43 * t45;
        let t49 = 1.0 + 0.252e-1 * t41 * t46;
        let t50 = 1.0 / t49;
        let t55 = 1.0 + 0.93333333333333333332e-3 * t22 * t34 * t39 * t50;
        let t56 = 1.0 / t55;
        let t60 = piecewise3(t14, 0.0, t22 * t32 * t56 / 9.0);
        let t64 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t65 = piecewise5(t26, t5, t24, t6, -t7);
        let t66 = 1.0 + t65;
        let t67 = t66 * rho[ip];
        let t68 = pow_1_3(t67);
        let t69 = 1.0 / t68;
        let t70 = t23 * t69;
        let t74 = piecewise3(t64, 0.0, t22 * t70 * t56 / 9.0);
        let t75 = t60 + t74;
        let t76 = t75 == 0.0;
        let t77 = piecewise3(t76, f64::EPSILON, t75);
        let t80 = 0.36011538e1 / t77 + 0.5764e0;
        let t81 = t77 * t77;
        let t82 = t81 * t81;
        let t83 = 1.0 / t82;
        let t85 = t81 * t77;
        let t86 = 1.0 / t85;
        let t88 = 1.0 / t81;
        let t90 = 0.31390124030721e2 * t83 + 0.149643497914092e2 * t86 + 0.17833359087e1 * t88;
        let t91 = 1.0 / t90;
        let tzk0 = piecewise3(t4, 0.0, -0.25e0 * t10 * t80 * t91);
        zk[ip] += tzk0;
        let t95 = t9 * t80;
        let t99 = 1.0 / t30 / t29;
        let t105 = t55 * t55;
        let t106 = 1.0 / t105;
        let t107 = t35 * rho[ip];
        let t109 = 1.0 / t37 / t107;
        let t114 = t21 * sigma[ip];
        let t115 = t20 * t114;
        let t116 = t33 * t39;
        let t117 = t49 * t49;
        let t118 = 1.0 / t117;
        let t121 = 1.0 / t36 / t35 * t45;
        let t125 = t34 * t39 + 1.0;
        let t126 = f64::sqrt(t125);
        let t127 = 1.0 / t126;
        let t128 = t109 * t127;
        let t131 = -0.336e-1 * t41 * t121 - 0.336e-1 * t34 * t128;
        let t132 = t118 * t131;
        let t133 = t116 * t132;
        let t136 = -0.24888888888888888889e-2 * t22 * t34 * t109 * t50 - 0.93333333333333333332e-3 * t115 * t133;
        let t137 = t106 * t136;
        let t142 = piecewise3(t14, 0.0, -t22 * t23 * t99 * t56 * t28 / 27.0 - t22 * t32 * t137 / 9.0);
        let t144 = 1.0 / t68 / t67;
        let t154 = piecewise3(t64, 0.0, -t22 * t23 * t144 * t56 * t66 / 27.0 - t22 * t70 * t137 / 9.0);
        let t156 = piecewise3(t76, 0.0, t142 + t154);
        let t161 = t90 * t90;
        let t162 = 1.0 / t161;
        let t163 = t80 * t162;
        let t165 = 1.0 / t82 / t77;
        let t166 = t165 * t156;
        let t168 = t83 * t156;
        let t172 = -0.125560496122884e3 * t166 - 0.448930493742276e2 * t168 - 0.35666718174e1 * t86 * t156;
        let t177 = piecewise3(t4, 0.0, -0.25e0 * t95 * t91 + 0.90028845e0 * t10 * t88 * t156 * t91 + 0.25e0 * t10 * t163 * t172);
        let tvrho0 = rho[ip] * t177 + tzk0;
        vrho[ip] += tvrho0;
        let t183 = 1.0 / t40 * t23;
        let t188 = 0.126e-1 * t183 * t46 + 0.126e-1 * t116 * t127;
        let t189 = t118 * t188;
        let t190 = t116 * t189;
        let t193 = 0.93333333333333333332e-3 * t22 * t116 * t50 - 0.93333333333333333332e-3 * t115 * t190;
        let t194 = t106 * t193;
        let t198 = piecewise3(t14, 0.0, -t22 * t32 * t194 / 9.0);
        let t202 = piecewise3(t64, 0.0, -t22 * t70 * t194 / 9.0);
        let t204 = piecewise3(t76, 0.0, t198 + t202);
        let t209 = t165 * t204;
        let t211 = t83 * t204;
        let t213 = t86 * t204;
        let t215 = -0.125560496122884e3 * t209 - 0.448930493742276e2 * t211 - 0.35666718174e1 * t213;
        let t220 = piecewise3(t4, 0.0, 0.90028845e0 * t10 * t88 * t204 * t91 + 0.25e0 * t10 * t163 * t215);
        let tvsigma0 = rho[ip] * t220;
        vsigma[ip] += tvsigma0;
        let t222 = t9 * t88;
        let t223 = t156 * t91;
        let t229 = t156 * t156;
        let t234 = t28 * t28;
        let t237 = 1.0 / t30 / t234 / t35;
        let t244 = t20 * t21 * t23;
        let t245 = t99 * t106;
        let t246 = t28 * t136;
        let t251 = 1.0 / t105 / t55;
        let t252 = t136 * t136;
        let t253 = t251 * t252;
        let t257 = t35 * t35;
        let t259 = 1.0 / t37 / t257;
        let t264 = t33 * t109;
        let t265 = t264 * t132;
        let t269 = 1.0 / t117 / t49;
        let t270 = t131 * t131;
        let t271 = t269 * t270;
        let t272 = t116 * t271;
        let t277 = 1.0 / t36 / t107 * t45;
        let t280 = t259 * t127;
        let t283 = sigma[ip] * sigma[ip];
        let t284 = t283 * t23;
        let t287 = 1.0 / t36 / t257 / t107;
        let t289 = 1.0 / t126 / t125;
        let t293 = 0.784e-1 * t41 * t277 + 0.168e0 * t34 * t280 - 0.896e-1 * t284 * t287 * t289;
        let t294 = t118 * t293;
        let t295 = t116 * t294;
        let t298 = 0.9125925925925925926e-2 * t22 * t34 * t259 * t50 + 0.49777777777777777778e-2 * t115 * t265 + 0.18666666666666666666e-2 * t115 * t272 - 0.93333333333333333332e-3 * t115 * t295;
        let t299 = t106 * t298;
        let t304 = piecewise3(t14, 0.0, 4.0 / 81.0 * t22 * t23 * t237 * t56 * t234 + 2.0 / 27.0 * t244 * t245 * t246 + 2.0 / 9.0 * t22 * t32 * t253 - t22 * t32 * t299 / 9.0);
        let t305 = t66 * t66;
        let t308 = 1.0 / t68 / t305 / t35;
        let t314 = t144 * t106;
        let t315 = t66 * t136;
        let t326 = piecewise3(t64, 0.0, 4.0 / 81.0 * t22 * t23 * t308 * t56 * t305 + 2.0 / 27.0 * t244 * t314 * t315 + 2.0 / 9.0 * t22 * t70 * t253 - t22 * t70 * t299 / 9.0);
        let t328 = piecewise3(t76, 0.0, t304 + t326);
        let t333 = t10 * t88;
        let t334 = t156 * t162;
        let t335 = t334 * t172;
        let t339 = 1.0 / t161 / t90;
        let t340 = t80 * t339;
        let t341 = t172 * t172;
        let t346 = 1.0 / t82 / t81;
        let t347 = t346 * t229;
        let t351 = t165 * t229;
        let t359 = 0.62780248061442e3 * t347 - 0.125560496122884e3 * t165 * t328 + 0.1795721974969104e3 * t351 - 0.448930493742276e2 * t83 * t328 + 0.107000154522e2 * t83 * t229 - 0.35666718174e1 * t86 * t328;
        let t364 = piecewise3(t4, 0.0, 0.18005769e1 * t222 * t223 + 0.5e0 * t95 * t162 * t172 - 0.18005769e1 * t10 * t86 * t229 * t91 + 0.90028845e0 * t10 * t88 * t328 * t91 - 0.18005769e1 * t333 * t335 - 0.5e0 * t10 * t340 * t341 + 0.25e0 * t10 * t163 * t359);
        let tv2rho20 = rho[ip] * t364 + 2.0 * t177;
        v2rho2[ip] += tv2rho20;
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
        let t401 = 1.0 / t36 / t399;
        let t402 = t23 * t401;
        let t403 = t289 * sigma[ip];
        let t406 = -0.168e-1 * t183 * t121 - 0.504e-1 * t264 * t127 + 0.336e-1 * t402 * t403;
        let t407 = t118 * t406;
        let t408 = t116 * t407;
        let t411 = -0.24888888888888888889e-2 * t22 * t264 * t50 - 0.93333333333333333332e-3 * t22 * t133 + 0.24888888888888888889e-2 * t115 * t387 + 0.18666666666666666666e-2 * t115 * t116 * t391 - 0.93333333333333333332e-3 * t115 * t408;
        let t412 = t106 * t411;
        let t417 = piecewise3(t14, 0.0, t244 * t245 * t373 / 27.0 + 2.0 / 9.0 * t244 * t377 * t378 - t22 * t32 * t412 / 9.0);
        let t418 = t193 * t66;
        let t422 = t69 * t251;
        let t430 = piecewise3(t64, 0.0, t244 * t314 * t418 / 27.0 + 2.0 / 9.0 * t244 * t422 * t378 - t22 * t70 * t412 / 9.0);
        let t432 = piecewise3(t76, 0.0, t417 + t430);
        let t437 = t204 * t162;
        let t438 = t437 * t172;
        let t444 = t334 * t215;
        let t447 = t10 * t80;
        let t448 = t339 * t215;
        let t449 = t448 * t172;
        let t452 = t346 * t204;
        let t455 = t165 * t432;
        let t459 = t83 * t432;
        let t465 = 0.62780248061442e3 * t452 * t156 - 0.125560496122884e3 * t455 + 0.1795721974969104e3 * t209 * t156 - 0.448930493742276e2 * t459 + 0.107000154522e2 * t211 * t156 - 0.35666718174e1 * t86 * t432;
        let t470 = piecewise3(t4, 0.0, 0.90028845e0 * t222 * t366 - 0.18005769e1 * t369 * t370 + 0.90028845e0 * t10 * t88 * t432 * t91 - 0.90028845e0 * t333 * t438 + 0.25e0 * t95 * t162 * t215 - 0.90028845e0 * t333 * t444 - 0.5e0 * t447 * t449 + 0.25e0 * t10 * t163 * t465);
        let tv2rhosigma0 = rho[ip] * t470 + t220;
        v2rhosigma[ip] += tv2rhosigma0;
        let t472 = t204 * t204;
        let t477 = t193 * t193;
        let t478 = t251 * t477;
        let t484 = t188 * t188;
        let t485 = t269 * t484;
        let t486 = t116 * t485;
        let t491 = 1.0 / t40 / sigma[ip] * t23;
        let t494 = 1.0 / sigma[ip];
        let t495 = t494 * t33;
        let t496 = t39 * t127;
        let t499 = t257 * rho[ip];
        let t501 = 1.0 / t36 / t499;
        let t505 = -0.63e-2 * t491 * t46 + 0.63e-2 * t495 * t496 - 0.126e-1 * t23 * t501 * t289;
        let t506 = t118 * t505;
        let t507 = t116 * t506;
        let t510 = -0.18666666666666666666e-2 * t22 * t190 + 0.18666666666666666666e-2 * t115 * t486 - 0.93333333333333333332e-3 * t115 * t507;
        let t511 = t106 * t510;
        let t516 = piecewise3(t14, 0.0, 2.0 / 9.0 * t22 * t32 * t478 - t22 * t32 * t511 / 9.0);
        let t524 = piecewise3(t64, 0.0, 2.0 / 9.0 * t22 * t70 * t478 - t22 * t70 * t511 / 9.0);
        let t526 = piecewise3(t76, 0.0, t516 + t524);
        let t531 = t437 * t215;
        let t534 = t215 * t215;
        let t538 = t346 * t472;
        let t540 = t165 * t526;
        let t542 = t165 * t472;
        let t544 = t83 * t526;
        let t550 = 0.62780248061442e3 * t538 - 0.125560496122884e3 * t540 + 0.1795721974969104e3 * t542 - 0.448930493742276e2 * t544 + 0.107000154522e2 * t83 * t472 - 0.35666718174e1 * t86 * t526;
        let t555 = piecewise3(t4, 0.0, -0.18005769e1 * t10 * t86 * t472 * t91 + 0.90028845e0 * t10 * t88 * t526 * t91 - 0.18005769e1 * t333 * t531 - 0.5e0 * t10 * t340 * t534 + 0.25e0 * t10 * t163 * t550);
        let tv2sigma20 = rho[ip] * t555;
        v2sigma2[ip] += tv2sigma20;
    }
}
