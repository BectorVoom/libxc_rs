//! GGA_X_N12 exc pol kernel.
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
pub fn gga_x_n12_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        let t17 = t16 * t7;
        let t18 = piecewise5::<f64>(t10, t11, t14, t15, t17);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3::<f64>(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3::<f64>(t19);
        let t25 = piecewise3::<f64>(t20, t22, t23 * t19);
        let t26 = pow_1_3::<f64>(t6);
        let t27 = t25 * t26;
        let t28 = param_CC_0_0;
        let t29 = param_CC_0_1;
        let t30 = t29 * sigma0;
        let t31 = rho0 * rho0;
        let t32 = pow_1_3::<f64>(rho0);
        let t33 = t32 * t32;
        let t35 = 1.0 / t33 / t31;
        let t38 = 1.0 + 0.4e-2 * sigma0 * t35;
        let t39 = 1.0 / t38;
        let t40 = t35 * t39;
        let t43 = param_CC_0_2;
        let t44 = sigma0 * sigma0;
        let t45 = t43 * t44;
        let t46 = t31 * t31;
        let t47 = t46 * rho0;
        let t49 = 1.0 / t32 / t47;
        let t50 = t38 * t38;
        let t51 = 1.0 / t50;
        let t52 = t49 * t51;
        let t55 = param_CC_0_3;
        let t56 = t44 * sigma0;
        let t57 = t55 * t56;
        let t58 = t46 * t46;
        let t59 = 1.0 / t58;
        let t60 = t50 * t38;
        let t61 = 1.0 / t60;
        let t62 = t59 * t61;
        let t65 = param_CC_1_0;
        let t66 = param_CC_1_1;
        let t67 = t66 * sigma0;
        let t70 = param_CC_1_2;
        let t71 = t70 * t44;
        let t74 = param_CC_1_3;
        let t75 = t74 * t56;
        let t78 = t65 + 0.4e-2 * t67 * t40 + 0.16e-4 * t71 * t52 + 0.64e-7 * t75 * t62;
        let t80 = M_CBRT2;
        let t81 = 1.0 / t26 * t80;
        let t83 = 1.0 + t17 <= zeta_threshold;
        let t85 = 1.0 - t17 <= zeta_threshold;
        let t86 = piecewise5::<f64>(t83, t11, t85, t15, t17);
        let t87 = 1.0 + t86;
        let t88 = t87 <= zeta_threshold;
        let t89 = 1.0 / t21;
        let t90 = pow_1_3::<f64>(t87);
        let t92 = piecewise3::<f64>(t88, t89, 1.0 / t90);
        let t95 = 1.0 + 0.39999999999999999998e0 * t81 * t92;
        let t96 = 1.0 / t95;
        let t98 = param_CC_2_0;
        let t99 = param_CC_2_1;
        let t100 = t99 * sigma0;
        let t103 = param_CC_2_2;
        let t104 = t103 * t44;
        let t107 = param_CC_2_3;
        let t108 = t107 * t56;
        let t111 = t98 + 0.4e-2 * t100 * t40 + 0.16e-4 * t104 * t52 + 0.64e-7 * t108 * t62;
        let t112 = t95 * t95;
        let t113 = 1.0 / t112;
        let t115 = param_CC_3_0;
        let t116 = param_CC_3_1;
        let t117 = t116 * sigma0;
        let t120 = param_CC_3_2;
        let t121 = t120 * t44;
        let t124 = param_CC_3_3;
        let t125 = t124 * t56;
        let t128 = t115 + 0.4e-2 * t117 * t40 + 0.16e-4 * t121 * t52 + 0.64e-7 * t125 * t62;
        let t129 = t112 * t95;
        let t130 = 1.0 / t129;
        let t132 = t28 + 0.4e-2 * t30 * t40 + 0.16e-4 * t45 * t52 + 0.64e-7 * t57 * t62 + t78 * t96 + t111 * t113 + t128 * t130;
        let t136 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t132);
        let t137 = rho1 <= dens_threshold;
        let t138 = -t16;
        let t140 = piecewise5::<f64>(t14, t11, t10, t15, t138 * t7);
        let t141 = 1.0 + t140;
        let t142 = t141 <= zeta_threshold;
        let t143 = pow_1_3::<f64>(t141);
        let t145 = piecewise3::<f64>(t142, t22, t143 * t141);
        let t146 = t145 * t26;
        let t147 = t29 * sigma2;
        let t148 = rho1 * rho1;
        let t149 = pow_1_3::<f64>(rho1);
        let t150 = t149 * t149;
        let t152 = 1.0 / t150 / t148;
        let t155 = 1.0 + 0.4e-2 * sigma2 * t152;
        let t156 = 1.0 / t155;
        let t157 = t152 * t156;
        let t160 = sigma2 * sigma2;
        let t161 = t43 * t160;
        let t162 = t148 * t148;
        let t163 = t162 * rho1;
        let t165 = 1.0 / t149 / t163;
        let t166 = t155 * t155;
        let t167 = 1.0 / t166;
        let t168 = t165 * t167;
        let t171 = t160 * sigma2;
        let t172 = t55 * t171;
        let t173 = t162 * t162;
        let t174 = 1.0 / t173;
        let t175 = t166 * t155;
        let t176 = 1.0 / t175;
        let t177 = t174 * t176;
        let t180 = t66 * sigma2;
        let t183 = t70 * t160;
        let t186 = t74 * t171;
        let t189 = t65 + 0.4e-2 * t180 * t157 + 0.16e-4 * t183 * t168 + 0.64e-7 * t186 * t177;
        let t190 = piecewise5::<f64>(t85, t11, t83, t15, -t17);
        let t191 = 1.0 + t190;
        let t192 = t191 <= zeta_threshold;
        let t193 = pow_1_3::<f64>(t191);
        let t195 = piecewise3::<f64>(t192, t89, 1.0 / t193);
        let t198 = 1.0 + 0.39999999999999999998e0 * t81 * t195;
        let t199 = 1.0 / t198;
        let t201 = t99 * sigma2;
        let t204 = t103 * t160;
        let t207 = t107 * t171;
        let t210 = t98 + 0.4e-2 * t201 * t157 + 0.16e-4 * t204 * t168 + 0.64e-7 * t207 * t177;
        let t211 = t198 * t198;
        let t212 = 1.0 / t211;
        let t214 = t116 * sigma2;
        let t217 = t120 * t160;
        let t220 = t124 * t171;
        let t223 = t115 + 0.4e-2 * t214 * t157 + 0.16e-4 * t217 * t168 + 0.64e-7 * t220 * t177;
        let t224 = t211 * t198;
        let t225 = 1.0 / t224;
        let t227 = t28 + 0.4e-2 * t147 * t157 + 0.16e-4 * t161 * t168 + 0.64e-7 * t172 * t177 + t189 * t199 + t210 * t212 + t223 * t225;
        let t231 = piecewise3::<f64>(t137, 0.0, -3.0 / 8.0 * t5 * t146 * t227);
        let tzk0 = t136 + t231;
        zk[ip] += tzk0;
    }
}
