//! GGA_X_SFAT exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_sfat_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_hyb_omega_0: f64,
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
        let t5 = 1.0 / t3 * t2;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * t7 * rho0 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * t7 * rho1 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t7 * t16);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = t25 * t5;
        let t27 = pow_1_3(t6);
        let t28 = t2 * t2;
        let t29 = t28 * M_PI;
        let t30 = 1.0 / M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = M_CBRT4;
        let t34 = t33 * t32;
        let t35 = t32 * t28;
        let t36 = t33 * t35;
        let t37 = rho0 * rho0;
        let t38 = pow_1_3(rho0);
        let t39 = t38 * t38;
        let t41 = 1.0 / t39 / t37;
        let t42 = t41 * sigma0;
        let t43 = f64::sqrt(sigma0);
        let t45 = 1.0 / t38 / rho0;
        let t46 = t45 * t43;
        let t47 = f64::ln(t46 + f64::sqrt(t46 * t46 + 1.0));
        let t50 = 1.0 + 0.252e-1 * t47 * t46;
        let t51 = 1.0 / t50;
        let t55 = 1.0 + 0.93333333333333333332e-3 * t51 * t42 * t36;
        let t58 = 1.0 / t55 * t34 * t29;
        let t59 = f64::sqrt(t58);
        let t61 = 1.0 / t59 * param_hyb_omega_0;
        let t62 = M_CBRT2;
        let t63 = t6 * t19;
        let t64 = pow_1_3(t63);
        let t65 = 1.0 / t64;
        let t66 = t65 * t62;
        let t68 = t66 * t61 / 2.0;
        let t69 = 0.192e1 <= t68;
        let t70 = 0.192e1 < t68;
        let t71 = piecewise3(t70, t68, 0.192e1);
        let t72 = t71 * t71;
        let t73 = t72 * t72;
        let t74 = 1.0 / t73;
        let t76 = t73 * t72;
        let t77 = 1.0 / t76;
        let t79 = t73 * t73;
        let t80 = 1.0 / t79;
        let t82 = t79 * t72;
        let t83 = 1.0 / t82;
        let t85 = t79 * t73;
        let t86 = 1.0 / t85;
        let t88 = t79 * t76;
        let t89 = 1.0 / t88;
        let t91 = t79 * t79;
        let t92 = 1.0 / t91;
        let t95 = 1.0 / t91 / t72;
        let t98 = 1.0 / t91 / t73;
        let t101 = 1.0 / t91 / t76;
        let t104 = 1.0 / t91 / t79;
        let t107 = 1.0 / t91 / t82;
        let t110 = 1.0 / t91 / t85;
        let t113 = 1.0 / t91 / t88;
        let t115 = t91 * t91;
        let t116 = 1.0 / t115;
        let t119 = 1.0 / t115 / t72;
        let t122 = 1.0 / t115 / t73;
        let t126 = -t74 / 30.0 + t77 / 70.0 - t80 / 135.0 + t83 / 231.0 - t86 / 364.0 + t89 / 540.0 - t92 / 765.0 + t95 / 1045.0 - t98 / 1386.0 + t101 / 1794.0 - t104 / 2275.0 + t107 / 2835.0 - t110 / 3480.0 + t113 / 4216.0 - t116 / 5049.0 + t119 / 5985.0 - t122 / 7030.0 + 1.0 / t72 / 9.0;
        let t127 = piecewise3(t70, 0.192e1, t68);
        let t128 = f64::atan2(1.0, t127);
        let t129 = t127 * t127;
        let t130 = t129 + 3.0;
        let t131 = 1.0 / t129;
        let t132 = 1.0 + t131;
        let t133 = f64::ln(t132);
        let t135 = -t133 * t130 + 1.0;
        let t138 = t128 + t135 * t127 / 4.0;
        let t142 = piecewise3(t69, t126, 1.0 - 8.0 / 3.0 * t138 * t127);
        let t143 = t142 * t27;
        let t144 = t55 * t143;
        let t147 = piecewise3(t1, 0.0, -3.0 / 8.0 * t144 * t26);
        let t148 = rho1 <= dens_threshold;
        let t149 = -t16;
        let t151 = piecewise5(t14, t11, t10, t15, t7 * t149);
        let t152 = 1.0 + t151;
        let t153 = t152 <= zeta_threshold;
        let t154 = pow_1_3(t152);
        let t156 = piecewise3(t153, t22, t154 * t152);
        let t157 = t156 * t5;
        let t158 = rho1 * rho1;
        let t159 = pow_1_3(rho1);
        let t160 = t159 * t159;
        let t162 = 1.0 / t160 / t158;
        let t163 = t162 * sigma2;
        let t164 = f64::sqrt(sigma2);
        let t166 = 1.0 / t159 / rho1;
        let t167 = t166 * t164;
        let t168 = f64::ln(t167 + f64::sqrt(t167 * t167 + 1.0));
        let t171 = 1.0 + 0.252e-1 * t168 * t167;
        let t172 = 1.0 / t171;
        let t176 = 1.0 + 0.93333333333333333332e-3 * t172 * t163 * t36;
        let t179 = 1.0 / t176 * t34 * t29;
        let t180 = f64::sqrt(t179);
        let t182 = 1.0 / t180 * param_hyb_omega_0;
        let t183 = t6 * t152;
        let t184 = pow_1_3(t183);
        let t185 = 1.0 / t184;
        let t186 = t185 * t62;
        let t188 = t186 * t182 / 2.0;
        let t189 = 0.192e1 <= t188;
        let t190 = 0.192e1 < t188;
        let t191 = piecewise3(t190, t188, 0.192e1);
        let t192 = t191 * t191;
        let t193 = t192 * t192;
        let t194 = 1.0 / t193;
        let t196 = t193 * t192;
        let t197 = 1.0 / t196;
        let t199 = t193 * t193;
        let t200 = 1.0 / t199;
        let t202 = t199 * t192;
        let t203 = 1.0 / t202;
        let t205 = t199 * t193;
        let t206 = 1.0 / t205;
        let t208 = t199 * t196;
        let t209 = 1.0 / t208;
        let t211 = t199 * t199;
        let t212 = 1.0 / t211;
        let t215 = 1.0 / t211 / t192;
        let t218 = 1.0 / t211 / t193;
        let t221 = 1.0 / t211 / t196;
        let t224 = 1.0 / t211 / t199;
        let t227 = 1.0 / t211 / t202;
        let t230 = 1.0 / t211 / t205;
        let t233 = 1.0 / t211 / t208;
        let t235 = t211 * t211;
        let t236 = 1.0 / t235;
        let t239 = 1.0 / t235 / t192;
        let t242 = 1.0 / t235 / t193;
        let t246 = -t194 / 30.0 + t197 / 70.0 - t200 / 135.0 + t203 / 231.0 - t206 / 364.0 + t209 / 540.0 - t212 / 765.0 + t215 / 1045.0 - t218 / 1386.0 + t221 / 1794.0 - t224 / 2275.0 + t227 / 2835.0 - t230 / 3480.0 + t233 / 4216.0 - t236 / 5049.0 + t239 / 5985.0 - t242 / 7030.0 + 1.0 / t192 / 9.0;
        let t247 = piecewise3(t190, 0.192e1, t188);
        let t248 = f64::atan2(1.0, t247);
        let t249 = t247 * t247;
        let t250 = t249 + 3.0;
        let t251 = 1.0 / t249;
        let t252 = 1.0 + t251;
        let t253 = f64::ln(t252);
        let t255 = -t253 * t250 + 1.0;
        let t258 = t248 + t255 * t247 / 4.0;
        let t262 = piecewise3(t189, t246, 1.0 - 8.0 / 3.0 * t258 * t247);
        let t263 = t262 * t27;
        let t264 = t176 * t263;
        let t267 = piecewise3(t148, 0.0, -3.0 / 8.0 * t264 * t157);
        let tzk0 = t147 + t267;
        zk[ip] += tzk0;
    }
}
