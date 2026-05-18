//! GGA_K_OL1 fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_ol1.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_ol1_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t19 = piecewise5::<f64>(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3::<f64>(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3::<f64>(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3::<f64>(t21, t24, t26 * t20);
        let t29 = pow_1_3::<f64>(t7);
        let t30 = t29 * t29;
        let t32 = rho0 * rho0;
        let t33 = pow_1_3::<f64>(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / t32;
        let t39 = M_CBRT2;
        let t40 = f64::sqrt(sigma0);
        let t41 = t39 * t40;
        let t43 = 1.0 / t33 / rho0;
        let t47 = M_CBRT6;
        let t49 = M_PI * M_PI;
        let t50 = pow_1_3::<f64>(t49);
        let t51 = t50 * t50;
        let t52 = 1.0 / t51;
        let t55 = 1.0 + 5.0 / 9.0 * (sigma0 * t36 / 72.0 + 0.677e-2 * t41 * t43) * t47 * t52;
        let t59 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t6 * t28 * t30 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t17;
        let t63 = piecewise5::<f64>(t15, t12, t11, t16, t61 * t8);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3::<f64>(t64);
        let t67 = t66 * t66;
        let t69 = piecewise3::<f64>(t65, t24, t67 * t64);
        let t71 = rho1 * rho1;
        let t72 = pow_1_3::<f64>(rho1);
        let t73 = t72 * t72;
        let t75 = 1.0 / t73 / t71;
        let t78 = f64::sqrt(sigma2);
        let t79 = t39 * t78;
        let t81 = 1.0 / t72 / rho1;
        let t88 = 1.0 + 5.0 / 9.0 * (sigma2 * t75 / 72.0 + 0.677e-2 * t79 * t81) * t47 * t52;
        let t92 = piecewise3::<f64>(t60, 0.0, 3.0 / 20.0 * t6 * t69 * t30 * t88);
        let tzk0 = t59 + t92;
        zk[ip] += tzk0;
        let t93 = t7 * t7;
        let t94 = 1.0 / t93;
        let t95 = t17 * t94;
        let t97 = piecewise5::<f64>(t11, 0.0, t15, 0.0, t8 - t95);
        let t100 = piecewise3::<f64>(t21, 0.0, 5.0 / 3.0 * t26 * t97);
        let t105 = 1.0 / t29;
        let t109 = t6 * t28 * t105 * t55 / 10.0;
        let t110 = t6 * t28;
        let t111 = t32 * rho0;
        let t113 = 1.0 / t34 / t111;
        let t117 = 1.0 / t33 / t32;
        let t120 = -sigma0 * t113 / 27.0 - 0.90266666666666666666e-2 * t41 * t117;
        let t122 = t47 * t52;
        let t123 = t30 * t120 * t122;
        let t127 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t6 * t100 * t30 * t55 + t109 + t110 * t123 / 12.0);
        let t128 = t61 * t94;
        let t130 = piecewise5::<f64>(t15, 0.0, t11, 0.0, -t8 - t128);
        let t133 = piecewise3::<f64>(t65, 0.0, 5.0 / 3.0 * t67 * t130);
        let t141 = t6 * t69 * t105 * t88 / 10.0;
        let t143 = piecewise3::<f64>(t60, 0.0, 3.0 / 20.0 * t6 * t133 * t30 * t88 + t141);
        let tvrho0 = t59 + t92 + t7 * (t127 + t143);
        vrho[ip * 2] += tvrho0;
        let t147 = piecewise5::<f64>(t11, 0.0, t15, 0.0, -t8 - t95);
        let t150 = piecewise3::<f64>(t21, 0.0, 5.0 / 3.0 * t26 * t147);
        let t156 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t6 * t150 * t30 * t55 + t109);
        let t158 = piecewise5::<f64>(t15, 0.0, t11, 0.0, t8 - t128);
        let t161 = piecewise3::<f64>(t65, 0.0, 5.0 / 3.0 * t67 * t158);
        let t166 = t6 * t69;
        let t167 = t71 * rho1;
        let t169 = 1.0 / t73 / t167;
        let t173 = 1.0 / t72 / t71;
        let t176 = -sigma2 * t169 / 27.0 - 0.90266666666666666666e-2 * t79 * t173;
        let t178 = t30 * t176 * t122;
        let t182 = piecewise3::<f64>(t60, 0.0, 3.0 / 20.0 * t6 * t161 * t30 * t88 + t141 + t166 * t178 / 12.0);
        let tvrho1 = t59 + t92 + t7 * (t156 + t182);
        vrho[ip * 2 + 1] += tvrho1;
        let t187 = t39 / t40;
        let t190 = t36 / 72.0 + 0.3385e-2 * t187 * t43;
        let t192 = t30 * t190 * t122;
        let t195 = piecewise3::<f64>(t1, 0.0, t110 * t192 / 12.0);
        let tvsigma0 = t7 * t195;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t198 = t39 / t78;
        let t201 = t75 / 72.0 + 0.3385e-2 * t198 * t81;
        let t203 = t30 * t201 * t122;
        let t206 = piecewise3::<f64>(t60, 0.0, t166 * t203 / 12.0);
        let tvsigma2 = t7 * t206;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t209 = 1.0 / t25;
        let t210 = t97 * t97;
        let t213 = t93 * t7;
        let t214 = 1.0 / t213;
        let t215 = t17 * t214;
        let t218 = piecewise5::<f64>(t11, 0.0, t15, 0.0, -2.0 * t94 + 2.0 * t215);
        let t222 = piecewise3::<f64>(t21, 0.0, 10.0 / 9.0 * t209 * t210 + 5.0 / 3.0 * t26 * t218);
        let t229 = t6 * t100 * t105 * t55;
        let t231 = t6 * t100;
        let t235 = 1.0 / t29 / t7;
        let t239 = t6 * t28 * t235 * t55 / 30.0;
        let t241 = t105 * t120 * t122;
        let t242 = t110 * t241;
        let t244 = t32 * t32;
        let t246 = 1.0 / t34 / t244;
        let t250 = 1.0 / t33 / t111;
        let t253 = 11.0 / 81.0 * sigma0 * t246 + 0.21062222222222222222e-1 * t41 * t250;
        let t255 = t30 * t253 * t122;
        let t259 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t6 * t222 * t30 * t55 + t229 / 5.0 + t231 * t123 / 6.0 - t239 + t242 / 9.0 + t110 * t255 / 12.0);
        let t260 = 1.0 / t66;
        let t261 = t130 * t130;
        let t264 = t61 * t214;
        let t267 = piecewise5::<f64>(t15, 0.0, t11, 0.0, 2.0 * t94 + 2.0 * t264);
        let t271 = piecewise3::<f64>(t65, 0.0, 10.0 / 9.0 * t260 * t261 + 5.0 / 3.0 * t67 * t267);
        let t278 = t6 * t133 * t105 * t88;
        let t283 = t6 * t69 * t235 * t88 / 30.0;
        let t285 = piecewise3::<f64>(t60, 0.0, 3.0 / 20.0 * t6 * t271 * t30 * t88 + t278 / 5.0 - t283);
        let tv2rho20 = 2.0 * t127 + 2.0 * t143 + t7 * (t259 + t285);
        v2rho2[ip * 3] += tv2rho20;
        let t288 = t209 * t147;
        let t292 = piecewise5::<f64>(t11, 0.0, t15, 0.0, 2.0 * t215);
        let t296 = piecewise3::<f64>(t21, 0.0, 10.0 / 9.0 * t288 * t97 + 5.0 / 3.0 * t26 * t292);
        let t303 = t6 * t150 * t105 * t55;
        let t305 = t6 * t150;
        let t311 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t6 * t296 * t30 * t55 + t303 / 10.0 + t305 * t123 / 12.0 + t229 / 10.0 - t239 + t242 / 18.0);
        let t312 = t260 * t158;
        let t316 = piecewise5::<f64>(t15, 0.0, t11, 0.0, 2.0 * t264);
        let t320 = piecewise3::<f64>(t65, 0.0, 10.0 / 9.0 * t312 * t130 + 5.0 / 3.0 * t67 * t316);
        let t327 = t6 * t161 * t105 * t88;
        let t330 = t6 * t133;
        let t334 = t105 * t176 * t122;
        let t335 = t166 * t334;
        let t338 = piecewise3::<f64>(t60, 0.0, 3.0 / 20.0 * t6 * t320 * t30 * t88 + t327 / 10.0 + t278 / 10.0 - t283 + t330 * t178 / 12.0 + t335 / 18.0);
        let tv2rho21 = t127 + t143 + t156 + t182 + t7 * (t311 + t338);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t343 = t147 * t147;
        let t348 = piecewise5::<f64>(t11, 0.0, t15, 0.0, 2.0 * t94 + 2.0 * t215);
        let t352 = piecewise3::<f64>(t21, 0.0, 10.0 / 9.0 * t209 * t343 + 5.0 / 3.0 * t26 * t348);
        let t359 = piecewise3::<f64>(t1, 0.0, 3.0 / 20.0 * t6 * t352 * t30 * t55 + t303 / 5.0 - t239);
        let t360 = t158 * t158;
        let t365 = piecewise5::<f64>(t15, 0.0, t11, 0.0, -2.0 * t94 + 2.0 * t264);
        let t369 = piecewise3::<f64>(t65, 0.0, 10.0 / 9.0 * t260 * t360 + 5.0 / 3.0 * t67 * t365);
        let t375 = t6 * t161;
        let t379 = t71 * t71;
        let t381 = 1.0 / t73 / t379;
        let t385 = 1.0 / t72 / t167;
        let t388 = 11.0 / 81.0 * sigma2 * t381 + 0.21062222222222222222e-1 * t79 * t385;
        let t390 = t30 * t388 * t122;
        let t394 = piecewise3::<f64>(t60, 0.0, 3.0 / 20.0 * t6 * t369 * t30 * t88 + t327 / 5.0 + t375 * t178 / 6.0 - t283 + t335 / 9.0 + t166 * t390 / 12.0);
        let tv2rho22 = 2.0 * t156 + 2.0 * t182 + t7 * (t359 + t394);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t400 = t105 * t190 * t122;
        let t402 = t110 * t400 / 18.0;
        let t406 = -t113 / 27.0 - 0.45133333333333333333e-2 * t187 * t117;
        let t408 = t30 * t406 * t122;
        let t412 = piecewise3::<f64>(t1, 0.0, t231 * t192 / 12.0 + t402 + t110 * t408 / 12.0);
        let tv2rhosigma0 = t7 * t412 + t195;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t417 = t105 * t201 * t122;
        let t419 = t166 * t417 / 18.0;
        let t421 = piecewise3::<f64>(t60, 0.0, t330 * t203 / 12.0 + t419);
        let tv2rhosigma2 = t7 * t421 + t206;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t426 = piecewise3::<f64>(t1, 0.0, t305 * t192 / 12.0 + t402);
        let tv2rhosigma3 = t7 * t426 + t195;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t433 = -t169 / 27.0 - 0.45133333333333333333e-2 * t198 * t173;
        let t435 = t30 * t433 * t122;
        let t439 = piecewise3::<f64>(t60, 0.0, t375 * t203 / 12.0 + t419 + t166 * t435 / 12.0);
        let tv2rhosigma5 = t7 * t439 + t206;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t441 = t3 * t28;
        let t442 = t30 * t39;
        let t443 = t441 * t442;
        let t445 = 1.0 / t40 / sigma0;
        let t447 = t445 * t43 * t122;
        let t450 = piecewise3::<f64>(t1, 0.0, -0.64895402177010868827e-3 * t443 * t447);
        let tv2sigma20 = t7 * t450;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t451 = t3 * t69;
        let t452 = t451 * t442;
        let t454 = 1.0 / t78 / sigma2;
        let t456 = t454 * t81 * t122;
        let t459 = piecewise3::<f64>(t60, 0.0, -0.64895402177010868827e-3 * t452 * t456);
        let tv2sigma25 = t7 * t459;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
