//! MGGA_X_FT98 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_ft98.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_ft98_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_a1: f64,
    param_a2: f64,
    param_b: f64,
    param_b1: f64,
    param_b2: f64,
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
        let t30 = rho0 * rho0;
        let t31 = pow_1_3(rho0);
        let t32 = t31 * t31;
        let t34 = 1.0 / t32 / t30;
        let t36 = param_a1 * sigma0 * t34 + 1.0;
        let t37 = f64::sqrt(t36);
        let t38 = param_a * t37;
        let t39 = param_b1 * sigma0;
        let t41 = t39 * t34 + 1.0;
        let t42 = pow_1_4(t41);
        let t43 = t42 * t42;
        let t44 = t43 * t42;
        let t45 = 1.0 / t44;
        let t46 = t45 * sigma0;
        let t49 = sigma0 * t34;
        let t51 = 1.0 / t32 / rho0;
        let t53 = -lapl0 * t51 + t49;
        let t54 = t53 * t53;
        let t55 = param_a2 * t54;
        let t56 = 1.0 + t49;
        let t57 = t56 * t56;
        let t58 = 1.0 / t57;
        let t61 = param_b * (t55 * t58 + 1.0);
        let t62 = param_b2 * param_b2;
        let t64 = f64::sqrt(t62 + 1.0);
        let t65 = t64 - param_b2;
        let t66 = sigma0 * sigma0;
        let t67 = t30 * t30;
        let t68 = t67 * rho0;
        let t70 = 1.0 / t31 / t68;
        let t71 = t66 * t70;
        let t72 = lapl0 * lapl0;
        let t73 = t30 * rho0;
        let t75 = 1.0 / t31 / t73;
        let t76 = t72 * t75;
        let t77 = t71 - t76 - param_b2;
        let t78 = pow_1_4(f64::EPSILON);
        let t79 = 1.0 / t78;
        let t80 = t77 < -t79;
        let t83 = 2.0 * param_b2;
        let t86 = t77 * t77;
        let t87 = t86 * t77;
        let t88 = 1.0 / t87;
        let t90 = t86 * t86;
        let t91 = t90 * t77;
        let t92 = 1.0 / t91;
        let t97 = piecewise3(0.0 < t77, t77, -t77);
        let t98 = t97 < t78;
        let t101 = t90 * t86;
        let t103 = t90 * t90;
        let t106 = -t79 < t77;
        let t107 = piecewise3(t106, t77, -t79);
        let t108 = t107 * t107;
        let t109 = 1.0 + t108;
        let t110 = f64::sqrt(t109);
        let t111 = t107 + t110;
        let t113 = piecewise5(t80, -2.0 * t71 + 2.0 * t76 + t83 - 1.0 / t77 / 2.0 + t88 / 8.0 - t92 / 16.0, t98, 1.0 - t71 + t76 + param_b2 + t86 / 2.0 - t90 / 8.0 + t101 / 16.0 - 5.0 / 128.0 * t103, 1.0 / t111);
        let t115 = t65 * t113 + 1.0;
        let t116 = M_CBRT2;
        let t117 = t116 - 1.0;
        let t118 = t117 * t65;
        let t120 = t118 * t113 + 1.0;
        let t121 = t120 * t120;
        let t122 = t121 * t120;
        let t123 = 1.0 / t122;
        let t124 = t115 * t123;
        let t125 = t124 * t54;
        let t127 = t38 * t46 * t34 + t61 * t125 + 1.0;
        let t128 = t3 * t3;
        let t129 = 1.0 / M_PI;
        let t130 = pow_1_3(t129);
        let t131 = t130 * t130;
        let t132 = t128 * t131;
        let t133 = M_CBRT4;
        let t134 = t132 * t133;
        let t135 = param_b * sigma0;
        let t139 = 1.0 + 81.0 / 4.0 * t134 * t135 * t34;
        let t140 = 1.0 / t139;
        let t141 = t127 * t140;
        let t142 = f64::sqrt(t141);
        let t146 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t142);
        let t147 = rho1 <= dens_threshold;
        let t148 = -t17;
        let t150 = piecewise5(t15, t12, t11, t16, t148 * t8);
        let t151 = 1.0 + t150;
        let t152 = t151 <= zeta_threshold;
        let t153 = pow_1_3(t151);
        let t155 = piecewise3(t152, t23, t153 * t151);
        let t156 = t155 * t27;
        let t157 = param_a1 * sigma2;
        let t158 = rho1 * rho1;
        let t159 = pow_1_3(rho1);
        let t160 = t159 * t159;
        let t162 = 1.0 / t160 / t158;
        let t164 = t157 * t162 + 1.0;
        let t165 = f64::sqrt(t164);
        let t166 = param_a * t165;
        let t169 = param_b1 * sigma2 * t162 + 1.0;
        let t170 = pow_1_4(t169);
        let t171 = t170 * t170;
        let t172 = t171 * t170;
        let t173 = 1.0 / t172;
        let t174 = t173 * sigma2;
        let t177 = sigma2 * t162;
        let t179 = 1.0 / t160 / rho1;
        let t181 = -lapl1 * t179 + t177;
        let t182 = t181 * t181;
        let t183 = param_a2 * t182;
        let t184 = 1.0 + t177;
        let t185 = t184 * t184;
        let t186 = 1.0 / t185;
        let t189 = param_b * (t183 * t186 + 1.0);
        let t190 = sigma2 * sigma2;
        let t191 = t158 * t158;
        let t192 = t191 * rho1;
        let t194 = 1.0 / t159 / t192;
        let t195 = t190 * t194;
        let t196 = lapl1 * lapl1;
        let t197 = t158 * rho1;
        let t199 = 1.0 / t159 / t197;
        let t200 = t196 * t199;
        let t201 = t195 - t200 - param_b2;
        let t202 = t201 < -t79;
        let t207 = t201 * t201;
        let t208 = t207 * t201;
        let t209 = 1.0 / t208;
        let t211 = t207 * t207;
        let t212 = t211 * t201;
        let t213 = 1.0 / t212;
        let t218 = piecewise3(0.0 < t201, t201, -t201);
        let t219 = t218 < t78;
        let t222 = t211 * t207;
        let t224 = t211 * t211;
        let t227 = -t79 < t201;
        let t228 = piecewise3(t227, t201, -t79);
        let t229 = t228 * t228;
        let t230 = 1.0 + t229;
        let t231 = f64::sqrt(t230);
        let t232 = t228 + t231;
        let t234 = piecewise5(t202, -2.0 * t195 + 2.0 * t200 + t83 - 1.0 / t201 / 2.0 + t209 / 8.0 - t213 / 16.0, t219, 1.0 - t195 + t200 + param_b2 + t207 / 2.0 - t211 / 8.0 + t222 / 16.0 - 5.0 / 128.0 * t224, 1.0 / t232);
        let t236 = t65 * t234 + 1.0;
        let t238 = t118 * t234 + 1.0;
        let t239 = t238 * t238;
        let t240 = t239 * t238;
        let t241 = 1.0 / t240;
        let t242 = t236 * t241;
        let t243 = t242 * t182;
        let t245 = t166 * t174 * t162 + t189 * t243 + 1.0;
        let t246 = param_b * sigma2;
        let t250 = 1.0 + 81.0 / 4.0 * t134 * t246 * t162;
        let t251 = 1.0 / t250;
        let t252 = t245 * t251;
        let t253 = f64::sqrt(t252);
        let t257 = piecewise3(t147, 0.0, -3.0 / 8.0 * t6 * t156 * t253);
        let tzk0 = t146 + t257;
        zk[ip] += tzk0;
    }
}
