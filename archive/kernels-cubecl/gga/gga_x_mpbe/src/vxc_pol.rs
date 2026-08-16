//! GGA_X_MPBE vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_mpbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_mpbe_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_a: f64,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
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
        let t18 = piecewise5::<f64>(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3::<f64>(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3::<f64>(t19);
        let t25 = piecewise3::<f64>(t20, t22, t23 * t19);
        let t26 = pow_1_3::<f64>(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_c1 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3::<f64>(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3::<f64>(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = param_a * t28;
        let t42 = t33 * sigma0;
        let t46 = 1.0 + t41 * t42 * t39 / 24.0;
        let t47 = 1.0 / t46;
        let t51 = t28 * t28;
        let t52 = param_c2 * t51;
        let t54 = 1.0 / t31 / t30;
        let t55 = t52 * t54;
        let t56 = sigma0 * sigma0;
        let t57 = t35 * t35;
        let t58 = t57 * rho0;
        let t60 = 1.0 / t36 / t58;
        let t62 = t46 * t46;
        let t63 = 1.0 / t62;
        let t67 = t30 * t30;
        let t68 = 1.0 / t67;
        let t69 = param_c3 * t68;
        let t70 = t56 * sigma0;
        let t71 = t57 * t57;
        let t72 = 1.0 / t71;
        let t74 = t62 * t46;
        let t75 = 1.0 / t74;
        let t79 = 1.0 + t34 * sigma0 * t39 * t47 / 24.0 + t55 * t56 * t60 * t63 / 576.0 + t69 * t70 * t72 * t75 / 2304.0;
        let t83 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t79);
        let t84 = rho1 <= dens_threshold;
        let t85 = -t16;
        let t87 = piecewise5::<f64>(t14, t11, t10, t15, t85 * t7);
        let t88 = 1.0 + t87;
        let t89 = t88 <= zeta_threshold;
        let t90 = pow_1_3::<f64>(t88);
        let t92 = piecewise3::<f64>(t89, t22, t90 * t88);
        let t93 = t92 * t26;
        let t94 = rho1 * rho1;
        let t95 = pow_1_3::<f64>(rho1);
        let t96 = t95 * t95;
        let t98 = 1.0 / t96 / t94;
        let t100 = t33 * sigma2;
        let t104 = 1.0 + t41 * t100 * t98 / 24.0;
        let t105 = 1.0 / t104;
        let t109 = sigma2 * sigma2;
        let t110 = t94 * t94;
        let t111 = t110 * rho1;
        let t113 = 1.0 / t95 / t111;
        let t115 = t104 * t104;
        let t116 = 1.0 / t115;
        let t120 = t109 * sigma2;
        let t121 = t110 * t110;
        let t122 = 1.0 / t121;
        let t124 = t115 * t104;
        let t125 = 1.0 / t124;
        let t129 = 1.0 + t34 * sigma2 * t98 * t105 / 24.0 + t55 * t109 * t113 * t116 / 576.0 + t69 * t120 * t122 * t125 / 2304.0;
        let t133 = piecewise3::<f64>(t84, 0.0, -3.0 / 8.0 * t5 * t93 * t129);
        let tzk0 = t83 + t133;
        zk[ip] += tzk0;
        let t134 = t6 * t6;
        let t135 = 1.0 / t134;
        let t136 = t16 * t135;
        let t138 = piecewise5::<f64>(t10, 0.0, t14, 0.0, t7 - t136);
        let t141 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t138);
        let t142 = t141 * t26;
        let t146 = t26 * t26;
        let t147 = 1.0 / t146;
        let t148 = t25 * t147;
        let t151 = t5 * t148 * t79 / 8.0;
        let t152 = t35 * rho0;
        let t154 = 1.0 / t37 / t152;
        let t160 = param_c1 * t51 * t54;
        let t161 = t57 * t35;
        let t163 = 1.0 / t36 / t161;
        let t164 = t56 * t163;
        let t165 = t63 * param_a;
        let t172 = param_c2 * t68;
        let t173 = t172 * t70;
        let t174 = t71 * rho0;
        let t175 = 1.0 / t174;
        let t176 = t175 * t75;
        let t177 = t176 * param_a;
        let t184 = t56 * t56;
        let t185 = t71 * t152;
        let t187 = 1.0 / t37 / t185;
        let t190 = t62 * t62;
        let t191 = 1.0 / t190;
        let t193 = t28 * t33;
        let t194 = t191 * param_a * t193;
        let t197 = -t34 * sigma0 * t154 * t47 / 9.0 + t160 * t164 * t165 / 216.0 - t55 * t164 * t63 / 108.0 + t173 * t177 / 432.0 - t69 * t70 * t175 * t75 / 288.0 + t69 * t184 * t187 * t194 / 6912.0;
        let t202 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t142 * t79 - t151 - 3.0 / 8.0 * t5 * t27 * t197);
        let t203 = t85 * t135;
        let t205 = piecewise5::<f64>(t14, 0.0, t10, 0.0, -t7 - t203);
        let t208 = piecewise3::<f64>(t89, 0.0, 4.0 / 3.0 * t90 * t205);
        let t209 = t208 * t26;
        let t213 = t92 * t147;
        let t216 = t5 * t213 * t129 / 8.0;
        let t218 = piecewise3::<f64>(t84, 0.0, -3.0 / 8.0 * t5 * t209 * t129 - t216);
        let tvrho0 = t83 + t133 + t6 * (t202 + t218);
        vrho[ip * 2] += tvrho0;
        let t222 = piecewise5::<f64>(t10, 0.0, t14, 0.0, -t7 - t136);
        let t225 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t222);
        let t226 = t225 * t26;
        let t231 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t226 * t79 - t151);
        let t233 = piecewise5::<f64>(t14, 0.0, t10, 0.0, t7 - t203);
        let t236 = piecewise3::<f64>(t89, 0.0, 4.0 / 3.0 * t90 * t233);
        let t237 = t236 * t26;
        let t241 = t94 * rho1;
        let t243 = 1.0 / t96 / t241;
        let t248 = t110 * t94;
        let t250 = 1.0 / t95 / t248;
        let t251 = t109 * t250;
        let t252 = t116 * param_a;
        let t259 = t172 * t120;
        let t260 = t121 * rho1;
        let t261 = 1.0 / t260;
        let t262 = t261 * t125;
        let t263 = t262 * param_a;
        let t270 = t109 * t109;
        let t271 = t121 * t241;
        let t273 = 1.0 / t96 / t271;
        let t276 = t115 * t115;
        let t277 = 1.0 / t276;
        let t279 = t277 * param_a * t193;
        let t282 = -t34 * sigma2 * t243 * t105 / 9.0 + t160 * t251 * t252 / 216.0 - t55 * t251 * t116 / 108.0 + t259 * t263 / 432.0 - t69 * t120 * t261 * t125 / 288.0 + t69 * t270 * t273 * t279 / 6912.0;
        let t287 = piecewise3::<f64>(t84, 0.0, -3.0 / 8.0 * t5 * t237 * t129 - t216 - 3.0 / 8.0 * t5 * t93 * t282);
        let tvrho1 = t83 + t133 + t6 * (t231 + t287);
        vrho[ip * 2 + 1] += tvrho1;
        let t294 = sigma0 * t60;
        let t301 = t172 * t56;
        let t302 = t72 * t75;
        let t303 = t302 * param_a;
        let t310 = t71 * t35;
        let t312 = 1.0 / t37 / t310;
        let t317 = t29 * t33 * t39 * t47 / 24.0 - t160 * t294 * t165 / 576.0 + t55 * t294 * t63 / 288.0 - t301 * t303 / 1152.0 + t69 * t56 * t72 * t75 / 768.0 - t69 * t70 * t312 * t194 / 18432.0;
        let t321 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t317);
        let tvsigma0 = t6 * t321;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t326 = sigma2 * t113;
        let t333 = t172 * t109;
        let t334 = t122 * t125;
        let t335 = t334 * param_a;
        let t342 = t121 * t94;
        let t344 = 1.0 / t96 / t342;
        let t349 = t29 * t33 * t98 * t105 / 24.0 - t160 * t326 * t252 / 576.0 + t55 * t326 * t116 / 288.0 - t333 * t335 / 1152.0 + t69 * t109 * t122 * t125 / 768.0 - t69 * t120 * t344 * t279 / 18432.0;
        let t353 = piecewise3::<f64>(t84, 0.0, -3.0 / 8.0 * t5 * t93 * t349);
        let tvsigma2 = t6 * t353;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
