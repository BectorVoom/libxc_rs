//! MGGA_X_MGGAC exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mggac.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::mbrxc::xc_mgga_x_mbrxc_get_x;

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mggac_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
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
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = pow_1_3(32.0);
        let t30 = t29 * t29;
        let t31 = t4 * t4;
        let t32 = t30 * t31;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / rho0;
        let t37 = tau0 * t36;
        let t38 = M_CBRT6;
        let t39 = M_PI * M_PI;
        let t40 = pow_1_3(t39);
        let t41 = t40 * t40;
        let t42 = 1.0 / t41;
        let t43 = t38 * t42;
        let t44 = 1.0 / rho0;
        let t45 = sigma0 * t44;
        let t46 = 1.0 / tau0;
        let t48 = t45 * t46 / 8.0;
        let t50 = 0.0 < 0.9999999999e0 - t48;
        let t52 = piecewise3(t50, 1.0 - t48, 0.1e-9);
        let t53 = t43 * t52;
        let t54 = t37 * t53;
        let t56 = tau0 * tau0;
        let t57 = rho0 * rho0;
        let t58 = t57 * rho0;
        let t60 = 1.0 / t33 / t58;
        let t61 = t56 * t60;
        let t62 = t38 * t38;
        let t64 = 1.0 / t40 / t39;
        let t65 = t62 * t64;
        let t66 = t52 * t52;
        let t67 = t65 * t66;
        let t68 = t61 * t67;
        let t70 = 1.0 + 0.55555555555555555556e-1 * t54 - 0.34860833333333333333e1 * t68;
        let t73 = 0.3712e1 + 0.11111111111111111111e1 * t54 + 0.11620277777777777778e1 * t68;
        let t74 = 1.0 / t73;
        let t78 = xc_mgga_x_mbrxc_get_x(t32 * t70 * t74 / 6.0);
        let t79 = pow_1_4(f64::EPSILON);
        let t80 = t78 < t79;
        let t81 = t29 * t4;
        let t82 = t3 * t3;
        let t83 = 1.0 / M_PI;
        let t84 = pow_1_3(t83);
        let t85 = 1.0 / t84;
        let t86 = t82 * t85;
        let t87 = M_CBRT4;
        let t89 = t81 * t86 * t87;
        let t90 = t89 / 12.0;
        let t91 = t81 * t82;
        let t92 = t85 * t87;
        let t93 = t78 * t78;
        let t94 = t92 * t93;
        let t97 = t93 * t78;
        let t98 = t92 * t97;
        let t101 = t93 * t93;
        let t102 = t92 * t101;
        let t105 = t101 * t78;
        let t106 = t92 * t105;
        let t109 = t101 * t93;
        let t110 = t92 * t109;
        let t118 = t81 * t86;
        let t119 = t79 < t78;
        let t120 = piecewise3(t119, t78, t79);
        let t122 = f64::exp(t120 / 3.0);
        let t123 = t87 * t122;
        let t124 = f64::exp(-t120);
        let t125 = t120 * t120;
        let t127 = t125 + 5.0 * t120 + 8.0;
        let t128 = t124 * t127;
        let t129 = 8.0 - t128;
        let t130 = 1.0 / t120;
        let t131 = t129 * t130;
        let t132 = 1.0 + t120;
        let t133 = pow_1_3(t132);
        let t134 = 1.0 / t133;
        let t135 = t131 * t134;
        let t139 = piecewise3(t80, -t90 - t91 * t94 / 108.0 + t91 * t98 / 108.0 - 13.0 / 1620.0 * t91 * t102 + 67.0 / 9720.0 * t91 * t106 - 52.0 / 8505.0 * t91 * t110 + 1811.0 / 326592.0 * t91 * t92 * t101 * t97, -t118 * t123 * t135 / 36.0);
        let t143 = piecewise3(t2, 0.0, 3.0 / 16.0 * t6 * t28 * t139);
        let t144 = rho1 <= dens_threshold;
        let t145 = -t17;
        let t147 = piecewise5(t15, t12, t11, t16, t145 * t8);
        let t148 = 1.0 + t147;
        let t149 = t148 <= zeta_threshold;
        let t150 = pow_1_3(t148);
        let t152 = piecewise3(t149, t23, t150 * t148);
        let t153 = t152 * t27;
        let t154 = pow_1_3(rho1);
        let t155 = t154 * t154;
        let t157 = 1.0 / t155 / rho1;
        let t158 = tau1 * t157;
        let t159 = 1.0 / rho1;
        let t160 = sigma2 * t159;
        let t161 = 1.0 / tau1;
        let t163 = t160 * t161 / 8.0;
        let t165 = 0.0 < 0.9999999999e0 - t163;
        let t167 = piecewise3(t165, 1.0 - t163, 0.1e-9);
        let t168 = t43 * t167;
        let t169 = t158 * t168;
        let t171 = tau1 * tau1;
        let t172 = rho1 * rho1;
        let t173 = t172 * rho1;
        let t175 = 1.0 / t154 / t173;
        let t176 = t171 * t175;
        let t177 = t167 * t167;
        let t178 = t65 * t177;
        let t179 = t176 * t178;
        let t181 = 1.0 + 0.55555555555555555556e-1 * t169 - 0.34860833333333333333e1 * t179;
        let t184 = 0.3712e1 + 0.11111111111111111111e1 * t169 + 0.11620277777777777778e1 * t179;
        let t185 = 1.0 / t184;
        let t189 = xc_mgga_x_mbrxc_get_x(t32 * t181 * t185 / 6.0);
        let t190 = t189 < t79;
        let t191 = t189 * t189;
        let t192 = t92 * t191;
        let t195 = t191 * t189;
        let t196 = t92 * t195;
        let t199 = t191 * t191;
        let t200 = t92 * t199;
        let t203 = t199 * t189;
        let t204 = t92 * t203;
        let t207 = t199 * t191;
        let t208 = t92 * t207;
        let t216 = t79 < t189;
        let t217 = piecewise3(t216, t189, t79);
        let t219 = f64::exp(t217 / 3.0);
        let t220 = t87 * t219;
        let t221 = f64::exp(-t217);
        let t222 = t217 * t217;
        let t224 = t222 + 5.0 * t217 + 8.0;
        let t225 = t221 * t224;
        let t226 = 8.0 - t225;
        let t227 = 1.0 / t217;
        let t228 = t226 * t227;
        let t229 = 1.0 + t217;
        let t230 = pow_1_3(t229);
        let t231 = 1.0 / t230;
        let t232 = t228 * t231;
        let t236 = piecewise3(t190, -t90 - t91 * t192 / 108.0 + t91 * t196 / 108.0 - 13.0 / 1620.0 * t91 * t200 + 67.0 / 9720.0 * t91 * t204 - 52.0 / 8505.0 * t91 * t208 + 1811.0 / 326592.0 * t91 * t92 * t199 * t195, -t118 * t220 * t232 / 36.0);
        let t240 = piecewise3(t144, 0.0, 3.0 / 16.0 * t6 * t153 * t236);
        let tzk0 = t143 + t240;
        zk[ip] += tzk0;
    }
}
