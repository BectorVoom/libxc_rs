//! GGA_X_N12 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_n12.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_n12_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_CC_0_0: f64,
    param_CC_0_1: f64,
    param_CC_0_2: f64,
    param_CC_0_3: f64,
    param_CC_1_0: f64,
    param_CC_1_1: f64,
    param_CC_1_2: f64,
    param_CC_1_3: f64,
    param_CC_2_0: f64,
    param_CC_2_1: f64,
    param_CC_2_2: f64,
    param_CC_2_3: f64,
    param_CC_3_0: f64,
    param_CC_3_1: f64,
    param_CC_3_2: f64,
    param_CC_3_3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t12 = t11 <= zeta_threshold;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t12, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t19 = t17 * t18;
        let t21 = param_CC_0_1;
        let t22 = t21 * sigma[ip];
        let t23 = M_CBRT2;
        let t24 = t23 * t23;
        let t25 = rho[ip] * rho[ip];
        let t26 = t18 * t18;
        let t28 = 1.0 / t26 / t25;
        let t29 = t24 * t28;
        let t33 = 1.0 + 0.4e-2 * sigma[ip] * t24 * t28;
        let t34 = 1.0 / t33;
        let t35 = t29 * t34;
        let t38 = param_CC_0_2;
        let t39 = sigma[ip] * sigma[ip];
        let t40 = t38 * t39;
        let t41 = t25 * t25;
        let t42 = t41 * rho[ip];
        let t44 = 1.0 / t18 / t42;
        let t46 = t33 * t33;
        let t47 = 1.0 / t46;
        let t48 = t23 * t44 * t47;
        let t51 = param_CC_0_3;
        let t52 = t39 * sigma[ip];
        let t53 = t51 * t52;
        let t54 = t41 * t41;
        let t55 = 1.0 / t54;
        let t56 = t46 * t33;
        let t57 = 1.0 / t56;
        let t58 = t55 * t57;
        let t62 = param_CC_1_1;
        let t63 = t62 * sigma[ip];
        let t66 = param_CC_1_2;
        let t67 = t66 * t39;
        let t70 = param_CC_1_3;
        let t71 = t70 * t52;
        let t74 = param_CC_1_0 + 0.4e-2 * t63 * t35 + 0.32e-4 * t67 * t48 + 0.256e-6 * t71 * t58;
        let t79 = piecewise3::<f64>(t12, 1.0 / t13, 1.0 / t15);
        let t82 = 1.0 + 0.39999999999999999998e0 / t18 * t23 * t79;
        let t83 = 1.0 / t82;
        let t86 = param_CC_2_1;
        let t87 = t86 * sigma[ip];
        let t90 = param_CC_2_2;
        let t91 = t90 * t39;
        let t94 = param_CC_2_3;
        let t95 = t94 * t52;
        let t98 = param_CC_2_0 + 0.4e-2 * t87 * t35 + 0.32e-4 * t91 * t48 + 0.256e-6 * t95 * t58;
        let t99 = t82 * t82;
        let t100 = 1.0 / t99;
        let t103 = param_CC_3_1;
        let t104 = t103 * sigma[ip];
        let t107 = param_CC_3_2;
        let t108 = t107 * t39;
        let t111 = param_CC_3_3;
        let t112 = t111 * t52;
        let t115 = param_CC_3_0 + 0.4e-2 * t104 * t35 + 0.32e-4 * t108 * t48 + 0.256e-6 * t112 * t58;
        let t116 = t99 * t82;
        let t117 = 1.0 / t116;
        let t119 = param_CC_0_0 + 0.4e-2 * t22 * t35 + 0.32e-4 * t40 * t48 + 0.256e-6 * t53 * t58 + t74 * t83 + t98 * t100 + t115 * t117;
        let t123 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t119);
        let tzk0 = 2.0 * t123;
        zk[ip] += tzk0;
        let t125 = t17 / t26;
        let t129 = t25 * rho[ip];
        let t131 = 1.0 / t26 / t129;
        let t132 = t24 * t131;
        let t133 = t132 * t34;
        let t136 = t21 * t39;
        let t137 = t41 * t25;
        let t139 = 1.0 / t18 / t137;
        let t141 = t23 * t139 * t47;
        let t146 = t38 * t52;
        let t147 = t54 * rho[ip];
        let t148 = 1.0 / t147;
        let t149 = t148 * t57;
        let t154 = t39 * t39;
        let t155 = t51 * t154;
        let t156 = t54 * t129;
        let t158 = 1.0 / t26 / t156;
        let t159 = t46 * t46;
        let t160 = 1.0 / t159;
        let t162 = t158 * t160 * t24;
        let t167 = t62 * t39;
        let t172 = t66 * t52;
        let t177 = t70 * t154;
        let t180 = -0.10666666666666666667e-1 * t63 * t133 + 0.85333333333333333336e-4 * t167 * t141 - 0.17066666666666666667e-3 * t67 * t141 + 0.13653333333333333334e-5 * t172 * t149 - 0.2048e-5 * t71 * t149 + 0.81920000000000000003e-8 * t177 * t162;
        let t182 = t74 * t100;
        let t186 = 1.0 / t18 / rho[ip] * t23 * t79;
        let t191 = t86 * t39;
        let t196 = t90 * t52;
        let t201 = t94 * t154;
        let t204 = -0.10666666666666666667e-1 * t87 * t133 + 0.85333333333333333336e-4 * t191 * t141 - 0.17066666666666666667e-3 * t91 * t141 + 0.13653333333333333334e-5 * t196 * t149 - 0.2048e-5 * t95 * t149 + 0.81920000000000000003e-8 * t201 * t162;
        let t206 = t98 * t117;
        let t211 = t103 * t39;
        let t216 = t107 * t52;
        let t221 = t111 * t154;
        let t224 = -0.10666666666666666667e-1 * t104 * t133 + 0.85333333333333333336e-4 * t211 * t141 - 0.17066666666666666667e-3 * t108 * t141 + 0.13653333333333333334e-5 * t216 * t149 - 0.2048e-5 * t112 * t149 + 0.81920000000000000003e-8 * t221 * t162;
        let t226 = t99 * t99;
        let t227 = 1.0 / t226;
        let t228 = t115 * t227;
        let t231 = -0.10666666666666666667e-1 * t22 * t133 + 0.85333333333333333336e-4 * t136 * t141 - 0.17066666666666666667e-3 * t40 * t141 + 0.13653333333333333334e-5 * t146 * t149 - 0.2048e-5 * t53 * t149 + 0.81920000000000000003e-8 * t155 * t162 + t180 * t83 + 0.13333333333333333333e0 * t182 * t186 + t204 * t100 + 0.26666666666666666666e0 * t206 * t186 + t224 * t117 + 0.39999999999999999999e0 * t228 * t186;
        let t236 = piecewise3::<f64>(t2, 0.0, -t6 * t125 * t119 / 8.0 - 3.0 / 8.0 * t6 * t19 * t231);
        let tvrho0 = 2.0 * rho[ip] * t236 + 2.0 * t123;
        vrho[ip] += tvrho0;
        let t239 = t21 * t24;
        let t240 = t28 * t34;
        let t245 = t38 * sigma[ip];
        let t250 = t51 * t39;
        let t253 = t54 * t25;
        let t255 = 1.0 / t26 / t253;
        let t257 = t255 * t160 * t24;
        let t260 = t62 * t24;
        let t265 = t66 * sigma[ip];
        let t270 = t70 * t39;
        let t275 = 0.4e-2 * t260 * t240 - 0.32e-4 * t63 * t48 + 0.64e-4 * t265 * t48 - 0.512e-6 * t67 * t58 + 0.768e-6 * t270 * t58 - 0.3072e-8 * t71 * t257;
        let t277 = t86 * t24;
        let t282 = t90 * sigma[ip];
        let t287 = t94 * t39;
        let t292 = 0.4e-2 * t277 * t240 - 0.32e-4 * t87 * t48 + 0.64e-4 * t282 * t48 - 0.512e-6 * t91 * t58 + 0.768e-6 * t287 * t58 - 0.3072e-8 * t95 * t257;
        let t294 = t103 * t24;
        let t299 = t107 * sigma[ip];
        let t304 = t111 * t39;
        let t309 = 0.4e-2 * t294 * t240 - 0.32e-4 * t104 * t48 + 0.64e-4 * t299 * t48 - 0.512e-6 * t108 * t58 + 0.768e-6 * t304 * t58 - 0.3072e-8 * t112 * t257;
        let t311 = 0.4e-2 * t239 * t240 - 0.32e-4 * t22 * t48 + 0.64e-4 * t245 * t48 - 0.512e-6 * t40 * t58 + 0.768e-6 * t250 * t58 - 0.3072e-8 * t53 * t257 + t275 * t83 + t292 * t100 + t309 * t117;
        let t315 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t311);
        let tvsigma0 = 2.0 * rho[ip] * t315;
        vsigma[ip] += tvsigma0;
    }
}
