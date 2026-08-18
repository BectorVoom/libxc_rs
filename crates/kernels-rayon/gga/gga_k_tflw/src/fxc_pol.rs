//! GGA_K_TFLW fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_tflw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_tflw_fxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_lambda: f64,
    param_gamma: f64,
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
        let t32 = param_lambda * sigma0;
        let t33 = rho0 * rho0;
        let t34 = pow_1_3(rho0);
        let t35 = t34 * t34;
        let t37 = 1.0 / t35 / t33;
        let t38 = M_CBRT6;
        let t40 = M_PI * M_PI;
        let t41 = pow_1_3(t40);
        let t42 = t41 * t41;
        let t43 = 1.0 / t42;
        let t47 = param_gamma + 5.0 / 72.0 * t32 * t37 * t38 * t43;
        let t51 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t47);
        let t52 = rho1 <= dens_threshold;
        let t53 = -t17;
        let t55 = piecewise5(t15, t12, t11, t16, t53 * t8);
        let t56 = 1.0 + t55;
        let t57 = t56 <= zeta_threshold;
        let t58 = pow_1_3(t56);
        let t59 = t58 * t58;
        let t61 = piecewise3(t57, t24, t59 * t56);
        let t62 = t61 * t30;
        let t63 = param_lambda * sigma2;
        let t64 = rho1 * rho1;
        let t65 = pow_1_3(rho1);
        let t66 = t65 * t65;
        let t68 = 1.0 / t66 / t64;
        let t73 = param_gamma + 5.0 / 72.0 * t63 * t68 * t38 * t43;
        let t77 = piecewise3(t52, 0.0, 3.0 / 20.0 * t6 * t62 * t73);
        let tzk0 = t51 + t77;
        zk[ip] += tzk0;
        let t78 = t7 * t7;
        let t79 = 1.0 / t78;
        let t80 = t17 * t79;
        let t82 = piecewise5(t11, 0.0, t15, 0.0, t8 - t80);
        let t85 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t82);
        let t86 = t85 * t30;
        let t90 = 1.0 / t29;
        let t91 = t28 * t90;
        let t94 = t6 * t91 * t47 / 10.0;
        let t95 = t6 * t31;
        let t98 = 1.0 / t35 / t33 / rho0;
        let t101 = t32 * t98 * t38 * t43;
        let t105 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t86 * t47 + t94 - t95 * t101 / 36.0);
        let t106 = t53 * t79;
        let t108 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t106);
        let t111 = piecewise3(t57, 0.0, 5.0 / 3.0 * t59 * t108);
        let t112 = t111 * t30;
        let t116 = t61 * t90;
        let t119 = t6 * t116 * t73 / 10.0;
        let t121 = piecewise3(t52, 0.0, 3.0 / 20.0 * t6 * t112 * t73 + t119);
        let tvrho0 = t51 + t77 + t7 * (t105 + t121);
        vrho[ip * 2] += tvrho0;
        let t125 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t80);
        let t128 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t125);
        let t129 = t128 * t30;
        let t134 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t129 * t47 + t94);
        let t136 = piecewise5(t15, 0.0, t11, 0.0, t8 - t106);
        let t139 = piecewise3(t57, 0.0, 5.0 / 3.0 * t59 * t136);
        let t140 = t139 * t30;
        let t144 = t6 * t62;
        let t147 = 1.0 / t66 / t64 / rho1;
        let t150 = t63 * t147 * t38 * t43;
        let t154 = piecewise3(t52, 0.0, 3.0 / 20.0 * t6 * t140 * t73 + t119 - t144 * t150 / 36.0);
        let tvrho1 = t51 + t77 + t7 * (t134 + t154);
        vrho[ip * 2 + 1] += tvrho1;
        let t158 = t38 * t43;
        let t159 = param_lambda * t37 * t158;
        let t162 = piecewise3(t1, 0.0, t95 * t159 / 96.0);
        let tvsigma0 = t7 * t162;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t164 = param_lambda * t68 * t158;
        let t167 = piecewise3(t52, 0.0, t144 * t164 / 96.0);
        let tvsigma2 = t7 * t167;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t170 = 1.0 / t25;
        let t171 = t82 * t82;
        let t174 = t78 * t7;
        let t175 = 1.0 / t174;
        let t176 = t17 * t175;
        let t179 = piecewise5(t11, 0.0, t15, 0.0, -2.0 * t79 + 2.0 * t176);
        let t183 = piecewise3(t21, 0.0, 10.0 / 9.0 * t170 * t171 + 5.0 / 3.0 * t26 * t179);
        let t184 = t183 * t30;
        let t188 = t85 * t90;
        let t190 = t6 * t188 * t47;
        let t192 = t6 * t86;
        let t196 = 1.0 / t29 / t7;
        let t197 = t28 * t196;
        let t200 = t6 * t197 * t47 / 30.0;
        let t201 = t6 * t91;
        let t202 = t201 * t101;
        let t204 = t33 * t33;
        let t206 = 1.0 / t35 / t204;
        let t209 = t32 * t206 * t38 * t43;
        let t213 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t184 * t47 + t190 / 5.0 - t192 * t101 / 18.0 - t200 - t202 / 27.0 + 11.0 / 108.0 * t95 * t209);
        let t214 = 1.0 / t58;
        let t215 = t108 * t108;
        let t218 = t53 * t175;
        let t221 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t79 + 2.0 * t218);
        let t225 = piecewise3(t57, 0.0, 10.0 / 9.0 * t214 * t215 + 5.0 / 3.0 * t59 * t221);
        let t226 = t225 * t30;
        let t230 = t111 * t90;
        let t232 = t6 * t230 * t73;
        let t234 = t61 * t196;
        let t237 = t6 * t234 * t73 / 30.0;
        let t239 = piecewise3(t52, 0.0, 3.0 / 20.0 * t6 * t226 * t73 + t232 / 5.0 - t237);
        let tv2rho20 = 2.0 * t105 + 2.0 * t121 + t7 * (t213 + t239);
        v2rho2[ip * 3] += tv2rho20;
        let t242 = t170 * t125;
        let t246 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t176);
        let t250 = piecewise3(t21, 0.0, 10.0 / 9.0 * t242 * t82 + 5.0 / 3.0 * t26 * t246);
        let t251 = t250 * t30;
        let t255 = t128 * t90;
        let t257 = t6 * t255 * t47;
        let t259 = t6 * t129;
        let t265 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t251 * t47 + t257 / 10.0 - t259 * t101 / 36.0 + t190 / 10.0 - t200 - t202 / 54.0);
        let t266 = t214 * t136;
        let t270 = piecewise5(t15, 0.0, t11, 0.0, 2.0 * t218);
        let t274 = piecewise3(t57, 0.0, 10.0 / 9.0 * t266 * t108 + 5.0 / 3.0 * t59 * t270);
        let t275 = t274 * t30;
        let t279 = t139 * t90;
        let t281 = t6 * t279 * t73;
        let t284 = t6 * t112;
        let t287 = t6 * t116;
        let t288 = t287 * t150;
        let t291 = piecewise3(t52, 0.0, 3.0 / 20.0 * t6 * t275 * t73 + t281 / 10.0 + t232 / 10.0 - t237 - t284 * t150 / 36.0 - t288 / 54.0);
        let tv2rho21 = t105 + t121 + t134 + t154 + t7 * (t265 + t291);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t296 = t125 * t125;
        let t301 = piecewise5(t11, 0.0, t15, 0.0, 2.0 * t79 + 2.0 * t176);
        let t305 = piecewise3(t21, 0.0, 10.0 / 9.0 * t170 * t296 + 5.0 / 3.0 * t26 * t301);
        let t306 = t305 * t30;
        let t312 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t306 * t47 + t257 / 5.0 - t200);
        let t313 = t136 * t136;
        let t318 = piecewise5(t15, 0.0, t11, 0.0, -2.0 * t79 + 2.0 * t218);
        let t322 = piecewise3(t57, 0.0, 10.0 / 9.0 * t214 * t313 + 5.0 / 3.0 * t59 * t318);
        let t323 = t322 * t30;
        let t328 = t6 * t140;
        let t332 = t64 * t64;
        let t334 = 1.0 / t66 / t332;
        let t337 = t63 * t334 * t38 * t43;
        let t341 = piecewise3(t52, 0.0, 3.0 / 20.0 * t6 * t323 * t73 + t281 / 5.0 - t328 * t150 / 18.0 - t237 - t288 / 27.0 + 11.0 / 108.0 * t144 * t337);
        let tv2rho22 = 2.0 * t134 + 2.0 * t154 + t7 * (t312 + t341);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t347 = t201 * t159 / 144.0;
        let t349 = param_lambda * t98 * t158;
        let t353 = piecewise3(t1, 0.0, t192 * t159 / 96.0 + t347 - t95 * t349 / 36.0);
        let tv2rhosigma0 = t7 * t353 + t162;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t358 = t287 * t164 / 144.0;
        let t360 = piecewise3(t52, 0.0, t284 * t164 / 96.0 + t358);
        let tv2rhosigma2 = t7 * t360 + t167;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t365 = piecewise3(t1, 0.0, t259 * t159 / 96.0 + t347);
        let tv2rhosigma3 = t7 * t365 + t162;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t370 = param_lambda * t147 * t158;
        let t374 = piecewise3(t52, 0.0, t328 * t164 / 96.0 + t358 - t144 * t370 / 36.0);
        let tv2rhosigma5 = t7 * t374 + t167;
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
