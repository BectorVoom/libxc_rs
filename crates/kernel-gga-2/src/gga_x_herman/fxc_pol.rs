//! GGA_X_HERMAN fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_herman.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_herman_fxc_pol(
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
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = t2 * t2;
        let t30 = pow_1_3(1.0 / M_PI);
        let t31 = 1.0 / t30;
        let t32 = t28 * t31;
        let t33 = M_CBRT4;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t43 = 1.0 + 0.66666666666666666668e-3 * t32 * t34 * t39;
        let t47 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t43);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t16;
        let t51 = piecewise5(t14, t11, t10, t15, t49 * t7);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t22, t54 * t52);
        let t57 = t56 * t26;
        let t58 = t33 * sigma2;
        let t59 = rho1 * rho1;
        let t60 = pow_1_3(rho1);
        let t61 = t60 * t60;
        let t63 = 1.0 / t61 / t59;
        let t67 = 1.0 + 0.66666666666666666668e-3 * t32 * t58 * t63;
        let t71 = piecewise3(t48, 0.0, -3.0 / 8.0 * t5 * t57 * t67);
        let tzk0 = t47 + t71;
        zk[ip] += tzk0;
        let t72 = t6 * t6;
        let t73 = 1.0 / t72;
        let t74 = t16 * t73;
        let t76 = piecewise5(t10, 0.0, t14, 0.0, t7 - t74);
        let t79 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t76);
        let t80 = t79 * t26;
        let t84 = t26 * t26;
        let t85 = 1.0 / t84;
        let t86 = t25 * t85;
        let t89 = t5 * t86 * t43 / 8.0;
        let t90 = t27 * t31;
        let t93 = 1.0 / t37 / t35 / rho0;
        let t94 = t34 * t93;
        let t98 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t80 * t43 - t89 + 0.13655681265105913629e-2 * t90 * t94);
        let t99 = t49 * t73;
        let t101 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t99);
        let t104 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t101);
        let t105 = t104 * t26;
        let t109 = t56 * t85;
        let t112 = t5 * t109 * t67 / 8.0;
        let t114 = piecewise3(t48, 0.0, -3.0 / 8.0 * t5 * t105 * t67 - t112);
        let tvrho0 = t47 + t71 + t6 * (t98 + t114);
        vrho[ip * 2] += tvrho0;
        let t118 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t74);
        let t121 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t118);
        let t122 = t121 * t26;
        let t127 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t122 * t43 - t89);
        let t129 = piecewise5(t14, 0.0, t10, 0.0, t7 - t99);
        let t132 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t129);
        let t133 = t132 * t26;
        let t137 = t57 * t31;
        let t140 = 1.0 / t61 / t59 / rho1;
        let t141 = t58 * t140;
        let t145 = piecewise3(t48, 0.0, -3.0 / 8.0 * t5 * t133 * t67 - t112 + 0.13655681265105913629e-2 * t137 * t141);
        let tvrho1 = t47 + t71 + t6 * (t127 + t145);
        vrho[ip * 2 + 1] += tvrho1;
        let t148 = t31 * t33;
        let t149 = t148 * t39;
        let t152 = piecewise3(t1, 0.0, -0.51208804744147176112e-3 * t27 * t149);
        let tvsigma0 = t6 * t152;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t153 = t148 * t63;
        let t156 = piecewise3(t48, 0.0, -0.51208804744147176112e-3 * t57 * t153);
        let tvsigma2 = t6 * t156;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t159 = t23 * t23;
        let t160 = 1.0 / t159;
        let t161 = t76 * t76;
        let t164 = t72 * t6;
        let t165 = 1.0 / t164;
        let t166 = t16 * t165;
        let t169 = piecewise5(t10, 0.0, t14, 0.0, -2.0 * t73 + 2.0 * t166);
        let t173 = piecewise3(t20, 0.0, 4.0 / 9.0 * t160 * t161 + 4.0 / 3.0 * t23 * t169);
        let t174 = t173 * t26;
        let t178 = t79 * t85;
        let t180 = t5 * t178 * t43;
        let t182 = t80 * t31;
        let t186 = 1.0 / t84 / t6;
        let t187 = t25 * t186;
        let t190 = t5 * t187 * t43 / 12.0;
        let t191 = t86 * t31;
        let t192 = t191 * t94;
        let t194 = t35 * t35;
        let t196 = 1.0 / t37 / t194;
        let t197 = t34 * t196;
        let t201 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t174 * t43 - t180 / 4.0 + 0.27311362530211827258e-2 * t182 * t94 + t190 + 0.91037875100706090861e-3 * t192 - 0.50070831305388349973e-2 * t90 * t197);
        let t202 = t54 * t54;
        let t203 = 1.0 / t202;
        let t204 = t101 * t101;
        let t207 = t49 * t165;
        let t210 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t73 + 2.0 * t207);
        let t214 = piecewise3(t53, 0.0, 4.0 / 9.0 * t203 * t204 + 4.0 / 3.0 * t54 * t210);
        let t215 = t214 * t26;
        let t219 = t104 * t85;
        let t221 = t5 * t219 * t67;
        let t223 = t56 * t186;
        let t226 = t5 * t223 * t67 / 12.0;
        let t228 = piecewise3(t48, 0.0, -3.0 / 8.0 * t5 * t215 * t67 - t221 / 4.0 + t226);
        let tv2rho20 = 2.0 * t98 + 2.0 * t114 + t6 * (t201 + t228);
        v2rho2[ip * 3] += tv2rho20;
        let t231 = t160 * t118;
        let t235 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t166);
        let t239 = piecewise3(t20, 0.0, 4.0 / 9.0 * t231 * t76 + 4.0 / 3.0 * t23 * t235);
        let t240 = t239 * t26;
        let t244 = t121 * t85;
        let t246 = t5 * t244 * t43;
        let t248 = t122 * t31;
        let t254 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t240 * t43 - t246 / 8.0 + 0.13655681265105913629e-2 * t248 * t94 - t180 / 8.0 + t190 + 0.45518937550353045431e-3 * t192);
        let t255 = t203 * t129;
        let t259 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t207);
        let t263 = piecewise3(t53, 0.0, 4.0 / 9.0 * t255 * t101 + 4.0 / 3.0 * t54 * t259);
        let t264 = t263 * t26;
        let t268 = t132 * t85;
        let t270 = t5 * t268 * t67;
        let t273 = t105 * t31;
        let t276 = t109 * t31;
        let t277 = t276 * t141;
        let t280 = piecewise3(t48, 0.0, -3.0 / 8.0 * t5 * t264 * t67 - t270 / 8.0 - t221 / 8.0 + t226 + 0.13655681265105913629e-2 * t273 * t141 + 0.4551893755035304543e-3 * t277);
        let tv2rho21 = t98 + t114 + t127 + t145 + t6 * (t254 + t280);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t285 = t118 * t118;
        let t290 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t73 + 2.0 * t166);
        let t294 = piecewise3(t20, 0.0, 4.0 / 9.0 * t160 * t285 + 4.0 / 3.0 * t23 * t290);
        let t295 = t294 * t26;
        let t301 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t295 * t43 - t246 / 4.0 + t190);
        let t302 = t129 * t129;
        let t307 = piecewise5(t14, 0.0, t10, 0.0, -2.0 * t73 + 2.0 * t207);
        let t311 = piecewise3(t53, 0.0, 4.0 / 9.0 * t203 * t302 + 4.0 / 3.0 * t54 * t307);
        let t312 = t311 * t26;
        let t317 = t133 * t31;
        let t321 = t59 * t59;
        let t323 = 1.0 / t61 / t321;
        let t324 = t58 * t323;
        let t328 = piecewise3(t48, 0.0, -3.0 / 8.0 * t5 * t312 * t67 - t270 / 4.0 + 0.27311362530211827258e-2 * t317 * t141 + t226 + 0.91037875100706090861e-3 * t277 - 0.50070831305388349973e-2 * t137 * t324);
        let tv2rho22 = 2.0 * t127 + 2.0 * t145 + t6 * (t301 + t328);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t334 = 0.17069601581382392037e-3 * t86 * t149;
        let t335 = t148 * t93;
        let t339 = piecewise3(t1, 0.0, -0.51208804744147176112e-3 * t80 * t149 - t334 + 0.1365568126510591363e-2 * t27 * t335);
        let tv2rhosigma0 = t6 * t339 + t152;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t344 = 0.17069601581382392037e-3 * t109 * t153;
        let t346 = piecewise3(t48, 0.0, -0.51208804744147176112e-3 * t105 * t153 - t344);
        let tv2rhosigma2 = t6 * t346 + t156;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t351 = piecewise3(t1, 0.0, -0.51208804744147176112e-3 * t122 * t149 - t334);
        let tv2rhosigma3 = t6 * t351 + t152;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t355 = t148 * t140;
        let t359 = piecewise3(t48, 0.0, -0.51208804744147176112e-3 * t133 * t153 - t344 + 0.1365568126510591363e-2 * t57 * t355);
        let tv2rhosigma5 = t6 * t359 + t156;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let tv2sigma20 = 0.0;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let tv2sigma25 = 0.0;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
