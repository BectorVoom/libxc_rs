//! GGA_K_PG fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_pg.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_pg_fxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_pg_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t36 = 1.0 / t35;
        let t37 = t32 * t36;
        let t38 = rho0 * rho0;
        let t39 = pow_1_3(rho0);
        let t40 = t39 * t39;
        let t42 = 1.0 / t40 / t38;
        let t46 = param_pg_mu * t32;
        let t47 = t36 * sigma0;
        let t51 = rmath::exp(-t46 * t47 * t42 / 24.0);
        let t52 = 5.0 / 72.0 * t37 * sigma0 * t42 + t51;
        let t56 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t52);
        let t57 = rho1 <= dens_threshold;
        let t58 = -t17;
        let t60 = piecewise5(t15, t12, t11, t16, t58 * t8);
        let t61 = 1.0 + t60;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t64 = t63 * t63;
        let t66 = piecewise3(t62, t24, t64 * t61);
        let t67 = t66 * t30;
        let t68 = rho1 * rho1;
        let t69 = pow_1_3(rho1);
        let t70 = t69 * t69;
        let t72 = 1.0 / t70 / t68;
        let t76 = t36 * sigma2;
        let t80 = rmath::exp(-t46 * t76 * t72 / 24.0);
        let t81 = 5.0 / 72.0 * t37 * sigma2 * t72 + t80;
        let t85 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t67 * t81);
        let tzk0 = t56 + t85;
        zk[ip] += tzk0;
        let t86 = t7 * t7;
        let t87 = 1.0 / t86;
        let t88 = t17 * t87;
        let t90 = piecewise5(t11, 0.0, t15, 0.0, t8 - t88);
        let t93 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t90);
        let t94 = t93 * t30;
        let t98 = 1.0 / t29;
        let t99 = t28 * t98;
        let t102 = t6 * t99 * t52 / 10.0;
        let t103 = t38 * rho0;
        let t105 = 1.0 / t40 / t103;
        let t106 = sigma0 * t105;
        let t109 = t46 * t36;
        let t113 = -5.0 / 27.0 * t37 * t106 + t109 * t106 * t51 / 9.0;
        let t118 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t94 * t52 + t102 + 3.0 / 20.0 * t6 * t31 * t113);
        let t119 = t58 * t87;
        let t121 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t119);
        let t124 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t121);
        let t125 = t124 * t30;
        let t129 = t66 * t98;
        let t132 = t6 * t129 * t81 / 10.0;
        let t134 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t125 * t81 + t132);
        let tvrho0 = t56 + t85 + t7 * (t118 + t134);
        vrho[ip * 2] += tvrho0;
        let t138 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t88);
        let t141 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t138);
        let t142 = t141 * t30;
        let t147 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t142 * t52 + t102);
        let t149 = piecewise5(t15, 0.0, t11, 0.0, t8 - t119);
        let t152 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t149);
        let t153 = t152 * t30;
        let t157 = t68 * rho1;
        let t159 = 1.0 / t70 / t157;
        let t160 = sigma2 * t159;
        let t166 = -5.0 / 27.0 * t37 * t160 + t109 * t160 * t80 / 9.0;
        let t171 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t153 * t81 + t132 + 3.0 / 20.0 * t6 * t67 * t166);
        let tvrho1 = t56 + t85 + t7 * (t147 + t171);
        vrho[ip * 2 + 1] += tvrho1;
        let t180 = 5.0 / 72.0 * t37 * t42 - t46 * t36 * t42 * t51 / 24.0;
        let t184 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t180);
        let tvsigma0 = t7 * t184;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t191 = 5.0 / 72.0 * t37 * t72 - t46 * t36 * t72 * t80 / 24.0;
        let t195 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t67 * t191);
        let tvsigma2 = t7 * t195;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t198 = 1.0 / t25;
        let t199 = t90 * t90;
        let t202 = t86 * t7;
        let t203 = 1.0 / t202;
        let t204 = t17 * t203;
        let t207 = piecewise5(t11, 0.0, t15, 0.0, -2.0 * t87 + 2.0 * t204);
        let t211 = piecewise3(t21, 0.0, 10.0 / 9.0 * t198 * t199 + 5.0 / 3.0 * t26 * t207);
        let t212 = t211 * t30;
        let t216 = t93 * t98;
        let t218 = t6 * t216 * t52;
        let t224 = 1.0 / t29 / t7;
        let t225 = t28 * t224;
        let t228 = t6 * t225 * t52 / 30.0;
        let t230 = t6 * t99 * t113;
        let t232 = t38 * t38;
        let t234 = 1.0 / t40 / t232;
        let t235 = sigma0 * t234;
        let t241 = param_pg_mu * param_pg_mu;
        let t242 = t32 * t32;
        let t243 = t241 * t242;
        let t245 = 1.0 / t34 / t33;
        let t246 = t243 * t245;
        let t247 = sigma0 * sigma0;
        let t250 = 1.0 / t39 / t232 / t103;
        let t255 = 55.0 / 81.0 * t37 * t235 - 11.0 / 27.0 * t109 * t235 * t51 + t246 * t247 * t250 * t51 / 81.0;
        let t260 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t212 * t52 + t218 / 5.0 + 3.0 / 10.0 * t6 * t94 * t113 - t228 + t230 / 5.0 + 3.0 / 20.0 * t6 * t31 * t255);
        let t261 = 1.0 / t63;
        let t262 = t121 * t121;
        let t265 = t58 * t203;
        let t268 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t87 + 2.0 * t265);
        let t272 = piecewise3(t62, 0.0, 10.0 / 9.0 * t261 * t262 + 5.0 / 3.0 * t64 * t268);
        let t273 = t272 * t30;
        let t277 = t124 * t98;
        let t279 = t6 * t277 * t81;
        let t281 = t66 * t224;
        let t284 = t6 * t281 * t81 / 30.0;
        let t286 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t273 * t81 + t279 / 5.0 - t284);
        let tv2rho20 = 2.0 * t118 + 2.0 * t134 + t7 * (t260 + t286);
        v2rho2[ip * 3] += tv2rho20;
        let t289 = t198 * t138;
        let t293 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t204);
        let t297 = piecewise3(t21, 0.0, 10.0 / 9.0 * t289 * t90 + 5.0 / 3.0 * t26 * t293);
        let t298 = t297 * t30;
        let t302 = t141 * t98;
        let t304 = t6 * t302 * t52;
        let t312 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t298 * t52 + t304 / 10.0 + 3.0 / 20.0 * t6 * t142 * t113 + t218 / 10.0 - t228 + t230 / 10.0);
        let t313 = t261 * t149;
        let t317 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t265);
        let t321 = piecewise3(t62, 0.0, 10.0 / 9.0 * t313 * t121 + 5.0 / 3.0 * t64 * t317);
        let t322 = t321 * t30;
        let t326 = t152 * t98;
        let t328 = t6 * t326 * t81;
        let t335 = t6 * t129 * t166;
        let t338 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t322 * t81 + t328 / 10.0 + t279 / 10.0 - t284 + 3.0 / 20.0 * t6 * t125 * t166 + t335 / 10.0);
        let tv2rho21 = t118 + t134 + t147 + t171 + t7 * (t312 + t338);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t343 = t138 * t138;
        let t348 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t87 + 2.0 * t204);
        let t352 = piecewise3(t21, 0.0, 10.0 / 9.0 * t198 * t343 + 5.0 / 3.0 * t26 * t348);
        let t353 = t352 * t30;
        let t359 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t353 * t52 + t304 / 5.0 - t228);
        let t360 = t149 * t149;
        let t365 = piecewise5(t15, 0.0, t11, 0.0, -2.0 * t87 + 2.0 * t265);
        let t369 = piecewise3(t62, 0.0, 10.0 / 9.0 * t261 * t360 + 5.0 / 3.0 * t64 * t365);
        let t370 = t369 * t30;
        let t379 = t68 * t68;
        let t381 = 1.0 / t70 / t379;
        let t382 = sigma2 * t381;
        let t388 = sigma2 * sigma2;
        let t391 = 1.0 / t69 / t379 / t157;
        let t396 = 55.0 / 81.0 * t37 * t382 - 11.0 / 27.0 * t109 * t382 * t80 + t246 * t388 * t391 * t80 / 81.0;
        let t401 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t370 * t81 + t328 / 5.0 + 3.0 / 10.0 * t6 * t153 * t166 - t284 + t335 / 5.0 + 3.0 / 20.0 * t6 * t67 * t396);
        let tv2rho22 = 2.0 * t147 + 2.0 * t171 + t7 * (t359 + t401);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t409 = t6 * t99 * t180 / 10.0;
        let t416 = t232 * t38;
        let t418 = 1.0 / t39 / t416;
        let t423 = -5.0 / 27.0 * t37 * t105 + t46 * t36 * t105 * t51 / 9.0 - t246 * t418 * sigma0 * t51 / 216.0;
        let t428 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t94 * t180 + t409 + 3.0 / 20.0 * t6 * t31 * t423);
        let tv2rhosigma0 = t7 * t428 + t184;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t435 = t6 * t129 * t191 / 10.0;
        let t437 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t125 * t191 + t435);
        let tv2rhosigma2 = t7 * t437 + t195;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t443 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t142 * t180 + t409);
        let tv2rhosigma3 = t7 * t443 + t184;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t454 = t379 * t68;
        let t456 = 1.0 / t69 / t454;
        let t461 = -5.0 / 27.0 * t37 * t159 + t46 * t36 * t159 * t80 / 9.0 - t246 * t456 * sigma2 * t80 / 216.0;
        let t466 = piecewise3(t57, 0.0, 3.0 / 20.0 * t6 * t153 * t191 + t435 + 3.0 / 20.0 * t6 * t67 * t461);
        let tv2rhosigma5 = t7 * t466 + t195;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t468 = t6 * t31;
        let t469 = t232 * rho0;
        let t474 = t243 * t245 / t39 / t469 * t51;
        let t477 = piecewise3(t1, 0.0, t468 * t474 / 3840.0);
        let tv2sigma20 = t7 * t477;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t478 = t6 * t67;
        let t479 = t379 * rho1;
        let t484 = t243 * t245 / t69 / t479 * t80;
        let t487 = piecewise3(t57, 0.0, t478 * t484 / 3840.0);
        let tv2sigma25 = t7 * t487;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
