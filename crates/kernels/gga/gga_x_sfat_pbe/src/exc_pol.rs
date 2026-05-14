//! GGA_X_SFAT_PBE exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat_pbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_sfat_pbe_exc_pol(
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
        let t35 = M_CBRT6;
        let t36 = M_PI * M_PI;
        let t37 = pow_1_3(t36);
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t40 = t39 * t35;
        let t41 = rho0 * rho0;
        let t42 = pow_1_3(rho0);
        let t43 = t42 * t42;
        let t45 = 1.0 / t43 / t41;
        let t49 = 0.804e0 + 0.91464571985215458336e-2 * t45 * sigma0 * t40;
        let t52 = 0.1804e1 - 0.646416e0 / t49;
        let t55 = 1.0 / t52 * t34 * t29;
        let t56 = f64::sqrt(t55);
        let t58 = 1.0 / t56 * param_hyb_omega_0;
        let t59 = M_CBRT2;
        let t60 = t6 * t19;
        let t61 = pow_1_3(t60);
        let t62 = 1.0 / t61;
        let t63 = t62 * t59;
        let t65 = t63 * t58 / 2.0;
        let t66 = 0.192e1 <= t65;
        let t67 = 0.192e1 < t65;
        let t68 = piecewise3(t67, t65, 0.192e1);
        let t69 = t68 * t68;
        let t70 = t69 * t69;
        let t71 = 1.0 / t70;
        let t73 = t70 * t69;
        let t74 = 1.0 / t73;
        let t76 = t70 * t70;
        let t77 = 1.0 / t76;
        let t79 = t76 * t69;
        let t80 = 1.0 / t79;
        let t82 = t76 * t70;
        let t83 = 1.0 / t82;
        let t85 = t76 * t73;
        let t86 = 1.0 / t85;
        let t88 = t76 * t76;
        let t89 = 1.0 / t88;
        let t92 = 1.0 / t88 / t69;
        let t95 = 1.0 / t88 / t70;
        let t98 = 1.0 / t88 / t73;
        let t101 = 1.0 / t88 / t76;
        let t104 = 1.0 / t88 / t79;
        let t107 = 1.0 / t88 / t82;
        let t110 = 1.0 / t88 / t85;
        let t112 = t88 * t88;
        let t113 = 1.0 / t112;
        let t116 = 1.0 / t112 / t69;
        let t119 = 1.0 / t112 / t70;
        let t123 = -t71 / 30.0 + t74 / 70.0 - t77 / 135.0 + t80 / 231.0 - t83 / 364.0 + t86 / 540.0 - t89 / 765.0 + t92 / 1045.0 - t95 / 1386.0 + t98 / 1794.0 - t101 / 2275.0 + t104 / 2835.0 - t107 / 3480.0 + t110 / 4216.0 - t113 / 5049.0 + t116 / 5985.0 - t119 / 7030.0 + 1.0 / t69 / 9.0;
        let t124 = piecewise3(t67, 0.192e1, t65);
        let t125 = f64::atan2(1.0, t124);
        let t126 = t124 * t124;
        let t127 = t126 + 3.0;
        let t128 = 1.0 / t126;
        let t129 = 1.0 + t128;
        let t130 = f64::ln(t129);
        let t132 = -t130 * t127 + 1.0;
        let t135 = t125 + t132 * t124 / 4.0;
        let t139 = piecewise3(t66, t123, 1.0 - 8.0 / 3.0 * t135 * t124);
        let t140 = t139 * t27;
        let t141 = t52 * t140;
        let t144 = piecewise3(t1, 0.0, -3.0 / 8.0 * t141 * t26);
        let t145 = rho1 <= dens_threshold;
        let t146 = -t16;
        let t148 = piecewise5(t14, t11, t10, t15, t7 * t146);
        let t149 = 1.0 + t148;
        let t150 = t149 <= zeta_threshold;
        let t151 = pow_1_3(t149);
        let t153 = piecewise3(t150, t22, t151 * t149);
        let t154 = t153 * t5;
        let t155 = rho1 * rho1;
        let t156 = pow_1_3(rho1);
        let t157 = t156 * t156;
        let t159 = 1.0 / t157 / t155;
        let t163 = 0.804e0 + 0.91464571985215458336e-2 * t159 * sigma2 * t40;
        let t166 = 0.1804e1 - 0.646416e0 / t163;
        let t169 = 1.0 / t166 * t34 * t29;
        let t170 = f64::sqrt(t169);
        let t172 = 1.0 / t170 * param_hyb_omega_0;
        let t173 = t6 * t149;
        let t174 = pow_1_3(t173);
        let t175 = 1.0 / t174;
        let t176 = t175 * t59;
        let t178 = t176 * t172 / 2.0;
        let t179 = 0.192e1 <= t178;
        let t180 = 0.192e1 < t178;
        let t181 = piecewise3(t180, t178, 0.192e1);
        let t182 = t181 * t181;
        let t183 = t182 * t182;
        let t184 = 1.0 / t183;
        let t186 = t183 * t182;
        let t187 = 1.0 / t186;
        let t189 = t183 * t183;
        let t190 = 1.0 / t189;
        let t192 = t189 * t182;
        let t193 = 1.0 / t192;
        let t195 = t189 * t183;
        let t196 = 1.0 / t195;
        let t198 = t189 * t186;
        let t199 = 1.0 / t198;
        let t201 = t189 * t189;
        let t202 = 1.0 / t201;
        let t205 = 1.0 / t201 / t182;
        let t208 = 1.0 / t201 / t183;
        let t211 = 1.0 / t201 / t186;
        let t214 = 1.0 / t201 / t189;
        let t217 = 1.0 / t201 / t192;
        let t220 = 1.0 / t201 / t195;
        let t223 = 1.0 / t201 / t198;
        let t225 = t201 * t201;
        let t226 = 1.0 / t225;
        let t229 = 1.0 / t225 / t182;
        let t232 = 1.0 / t225 / t183;
        let t236 = -t184 / 30.0 + t187 / 70.0 - t190 / 135.0 + t193 / 231.0 - t196 / 364.0 + t199 / 540.0 - t202 / 765.0 + t205 / 1045.0 - t208 / 1386.0 + t211 / 1794.0 - t214 / 2275.0 + t217 / 2835.0 - t220 / 3480.0 + t223 / 4216.0 - t226 / 5049.0 + t229 / 5985.0 - t232 / 7030.0 + 1.0 / t182 / 9.0;
        let t237 = piecewise3(t180, 0.192e1, t178);
        let t238 = f64::atan2(1.0, t237);
        let t239 = t237 * t237;
        let t240 = t239 + 3.0;
        let t241 = 1.0 / t239;
        let t242 = 1.0 + t241;
        let t243 = f64::ln(t242);
        let t245 = -t243 * t240 + 1.0;
        let t248 = t238 + t245 * t237 / 4.0;
        let t252 = piecewise3(t179, t236, 1.0 - 8.0 / 3.0 * t248 * t237);
        let t253 = t252 * t27;
        let t254 = t166 * t253;
        let t257 = piecewise3(t145, 0.0, -3.0 / 8.0 * t254 * t154);
        let tzk0 = t144 + t257;
        zk[ip] += tzk0;
    }
}
