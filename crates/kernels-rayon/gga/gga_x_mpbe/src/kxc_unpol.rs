//! GGA_X_MPBE kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_mpbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_mpbe_kxc_unpol(
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
    param_c1: f64,
    param_a: f64,
    param_c2: f64,
    param_c3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = param_c1 * t20 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = param_a * t20;
        let t39 = 1.0 + t34 * t25 * t29 * t33 / 24.0;
        let t40 = 1.0 / t39;
        let t45 = t20 * t20;
        let t48 = 1.0 / t23 / t22;
        let t49 = param_c2 * t45 * t48;
        let t50 = sigma[ip] * sigma[ip];
        let t51 = t50 * t27;
        let t52 = t30 * t30;
        let t53 = t52 * rho[ip];
        let t55 = 1.0 / t18 / t53;
        let t56 = t39 * t39;
        let t57 = 1.0 / t56;
        let t58 = t55 * t57;
        let t62 = t22 * t22;
        let t63 = 1.0 / t62;
        let t64 = param_c3 * t63;
        let t65 = t50 * sigma[ip];
        let t66 = t52 * t52;
        let t67 = 1.0 / t66;
        let t69 = t56 * t39;
        let t70 = 1.0 / t69;
        let t74 = 1.0 + t26 * t29 * t33 * t40 / 24.0 + t49 * t51 * t58 / 288.0 + t64 * t65 * t67 * t70 / 576.0;
        let t78 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t80 = t17 / t31;
        let t84 = t30 * rho[ip];
        let t86 = 1.0 / t31 / t84;
        let t91 = param_c1 * t45;
        let t93 = t91 * t48 * t50;
        let t94 = t52 * t30;
        let t96 = 1.0 / t18 / t94;
        let t97 = t27 * t96;
        let t98 = t57 * param_a;
        let t99 = t97 * t98;
        let t102 = t96 * t57;
        let t106 = param_c2 * t63;
        let t107 = t106 * t65;
        let t108 = t66 * rho[ip];
        let t109 = 1.0 / t108;
        let t110 = t109 * t70;
        let t111 = t110 * param_a;
        let t118 = t50 * t50;
        let t119 = t66 * t84;
        let t121 = 1.0 / t31 / t119;
        let t124 = t56 * t56;
        let t125 = 1.0 / t124;
        let t128 = t20 * t25 * t28;
        let t129 = t125 * param_a * t128;
        let t132 = -t26 * t29 * t86 * t40 / 9.0 + t93 * t99 / 108.0 - t49 * t51 * t102 / 54.0 + t107 * t111 / 108.0 - t64 * t65 * t109 * t70 / 72.0 + t64 * t118 * t121 * t129 / 1728.0;
        let t137 = piecewise3(t2, 0.0, -t6 * t80 * t74 / 8.0 - 3.0 / 8.0 * t6 * t19 * t132);
        let tvrho0 = 2.0 * rho[ip] * t137 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t146 = t27 * t55;
        let t147 = t146 * t98;
        let t150 = sigma[ip] * t27;
        let t154 = t106 * t50;
        let t155 = t67 * t70;
        let t156 = t155 * param_a;
        let t163 = t66 * t30;
        let t165 = 1.0 / t31 / t163;
        let t170 = t26 * t28 * t33 * t40 / 24.0 - t91 * t48 * sigma[ip] * t147 / 288.0 + t49 * t150 * t58 / 144.0 - t154 * t156 / 288.0 + t64 * t50 * t67 * t70 / 192.0 - t64 * t65 * t165 * t129 / 4608.0;
        let t174 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t170);
        let tvsigma0 = 2.0 * rho[ip] * t174;
        vsigma[ip] += tvsigma0;
        let t179 = t17 / t31 / rho[ip];
        let t187 = 1.0 / t31 / t52;
        let t192 = t52 * t84;
        let t194 = 1.0 / t18 / t192;
        let t195 = t27 * t194;
        let t196 = t195 * t98;
        let t199 = param_c1 * t63;
        let t200 = t199 * t65;
        let t201 = 1.0 / t163;
        let t202 = t201 * t70;
        let t203 = param_a * param_a;
        let t207 = t194 * t57;
        let t211 = t202 * param_a;
        let t214 = t66 * t52;
        let t216 = 1.0 / t31 / t214;
        let t217 = t118 * t216;
        let t220 = t125 * t203 * t128;
        let t230 = t118 * sigma[ip];
        let t233 = 1.0 / t18 / t66 / t192;
        let t237 = 1.0 / t124 / t39;
        let t240 = t45 * t48 * t27;
        let t241 = t237 * t203 * t240;
        let t244 = 11.0 / 27.0 * t26 * t29 * t187 * t40 - t93 * t196 / 12.0 + 2.0 / 81.0 * t200 * t202 * t203 + 19.0 / 162.0 * t49 * t51 * t207 - 43.0 / 324.0 * t107 * t211 + t106 * t217 * t220 / 324.0 + t64 * t65 * t201 * t70 / 8.0 - 59.0 / 5184.0 * t64 * t217 * t129 + t64 * t230 * t233 * t241 / 1944.0;
        let t249 = piecewise3(t2, 0.0, t6 * t179 * t74 / 12.0 - t6 * t80 * t132 / 4.0 - 3.0 / 8.0 * t6 * t19 * t244);
        let tv2rho20 = 2.0 * rho[ip] * t249 + 4.0 * t137;
        v2rho2[ip] += tv2rho20;
        let t260 = t91 * t48 * t27;
        let t261 = param_a * sigma[ip];
        let t266 = t110 * t203;
        let t274 = t65 * t121;
        let t285 = t66 * t94;
        let t287 = 1.0 / t18 / t285;
        let t292 = -t26 * t28 * t86 * t40 / 9.0 + t260 * t102 * t261 / 36.0 - t199 * t50 * t266 / 108.0 - t49 * t150 * t102 / 27.0 + 5.0 / 108.0 * t154 * t111 - t106 * t274 * t220 / 864.0 - t64 * t50 * t109 * t70 / 24.0 + 7.0 / 1728.0 * t64 * t274 * t129 - t64 * t118 * t287 * t241 / 5184.0;
        let t297 = piecewise3(t2, 0.0, -t6 * t80 * t170 / 8.0 - 3.0 / 8.0 * t6 * t19 * t292);
        let tv2rhosigma0 = 2.0 * rho[ip] * t297 + 2.0 * t174;
        v2rhosigma[ip] += tv2rhosigma0;
        let t300 = t91 * t48;
        let t304 = t155 * t203;
        let t313 = t50 * t165;
        let t324 = t66 * t53;
        let t326 = 1.0 / t18 / t324;
        let t331 = -t300 * t147 / 144.0 + t199 * sigma[ip] * t304 / 288.0 + t49 * t146 * t57 / 144.0 - t106 * sigma[ip] * t156 / 72.0 + t106 * t313 * t220 / 2304.0 + t64 * sigma[ip] * t67 * t70 / 96.0 - t64 * t313 * t129 / 768.0 + t64 * t65 * t326 * t241 / 13824.0;
        let t335 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t331);
        let tv2sigma20 = 2.0 * rho[ip] * t335;
        v2sigma2[ip] += tv2sigma20;
        let t338 = t17 * t33;
        let t349 = 1.0 / t31 / t53;
        let t355 = 1.0 / t18 / t66;
        let t360 = 1.0 / t119;
        let t361 = t360 * t70;
        let t366 = 1.0 / t31 / t324;
        let t367 = t118 * t366;
        let t369 = t203 * param_a;
        let t371 = t125 * t369 * t128;
        let t374 = t355 * t57;
        let t378 = t361 * param_a;
        let t384 = t66 * t66;
        let t386 = 1.0 / t18 / t384;
        let t387 = t230 * t386;
        let t390 = t237 * t369 * t240;
        let t403 = t62 * t62;
        let t404 = 1.0 / t403;
        let t405 = param_c3 * t404;
        let t406 = t118 * t50;
        let t407 = t405 * t406;
        let t408 = t384 * t84;
        let t411 = 1.0 / t124 / t56;
        let t412 = 1.0 / t408 * t411;
        let t413 = t412 * t369;
        let t416 = -154.0 / 81.0 * t26 * t29 * t349 * t40 + 341.0 / 486.0 * t93 * t27 * t355 * t98 - 38.0 / 81.0 * t200 * t361 * t203 + 2.0 / 243.0 * t199 * t367 * t371 - 209.0 / 243.0 * t49 * t51 * t374 + 797.0 / 486.0 * t107 * t378 - t106 * t367 * t220 / 12.0 + 2.0 / 729.0 * t106 * t387 * t390 - 5.0 / 4.0 * t64 * t65 * t360 * t70 + 1445.0 / 7776.0 * t64 * t367 * t129 - 35.0 / 1944.0 * t64 * t387 * t241 + 5.0 / 1458.0 * t407 * t413;
        let t421 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t338 * t74 + t6 * t179 * t132 / 4.0 - 3.0 / 8.0 * t6 * t80 * t244 - 3.0 / 8.0 * t6 * t19 * t416);
        let tv3rho30 = 2.0 * rho[ip] * t421 + 6.0 * t249;
        v3rho3[ip] += tv3rho30;
        let t438 = t199 * t201;
        let t439 = t70 * t203;
        let t440 = t439 * t50;
        let t443 = t65 * t216;
        let t455 = t118 * t233;
        let t469 = t405 * t230;
        let t470 = t384 * t30;
        let t472 = 1.0 / t470 * t411;
        let t473 = t472 * t369;
        let t476 = 11.0 / 27.0 * t26 * t28 * t187 * t40 - 65.0 / 324.0 * t260 * t207 * t261 + 17.0 / 108.0 * t438 * t440 - t199 * t443 * t371 / 324.0 + 19.0 / 81.0 * t49 * t150 * t207 - 167.0 / 324.0 * t154 * t211 + 25.0 / 864.0 * t106 * t443 * t220 - t106 * t455 * t390 / 972.0 + 3.0 / 8.0 * t64 * t50 * t201 * t70 - 317.0 / 5184.0 * t64 * t443 * t129 + 11.0 / 1728.0 * t64 * t455 * t241 - 5.0 / 3888.0 * t469 * t473;
        let t481 = piecewise3(t2, 0.0, t6 * t179 * t170 / 12.0 - t6 * t80 * t292 / 4.0 - 3.0 / 8.0 * t6 * t19 * t476);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t481 + 4.0 * t297;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t490 = t439 * sigma[ip];
        let t493 = t50 * t121;
        let t502 = t70 * param_a * sigma[ip];
        let t508 = t65 * t287;
        let t522 = t405 * t118;
        let t523 = t384 * rho[ip];
        let t525 = 1.0 / t523 * t411;
        let t526 = t525 * t369;
        let t529 = t300 * t99 / 27.0 - 5.0 / 108.0 * t199 * t109 * t490 + t199 * t493 * t371 / 864.0 - t49 * t97 * t57 / 27.0 + 7.0 / 54.0 * t106 * t109 * t502 - t106 * t493 * t220 / 108.0 + t106 * t508 * t390 / 2592.0 - t64 * sigma[ip] * t109 * t70 / 12.0 + 5.0 / 288.0 * t64 * t493 * t129 - 11.0 / 5184.0 * t64 * t508 * t241 + 5.0 / 10368.0 * t522 * t526;
        let t534 = piecewise3(t2, 0.0, -t6 * t80 * t331 / 8.0 - 3.0 / 8.0 * t6 * t19 * t529);
        let tv3rhosigma20 = 2.0 * rho[ip] * t534 + 2.0 * t335;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t539 = sigma[ip] * t165;
        let t548 = t50 * t326;
        let t560 = t405 * t65;
        let t562 = 1.0 / t384 * t411;
        let t563 = t562 * t369;
        let t566 = t199 * t304 / 96.0 - t199 * t539 * t371 / 2304.0 - t106 * t156 / 48.0 + t106 * t539 * t220 / 384.0 - t106 * t548 * t390 / 6912.0 + t64 * t155 / 96.0 - t64 * t539 * t129 / 256.0 + t64 * t548 * t241 / 1536.0 - 5.0 / 27648.0 * t560 * t563;
        let t570 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t566);
        let tv3sigma30 = 2.0 * rho[ip] * t570;
        v3sigma3[ip] += tv3sigma30;
    }
}
