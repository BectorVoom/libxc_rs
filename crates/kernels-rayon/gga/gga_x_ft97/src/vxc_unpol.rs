//! GGA_X_FT97 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ft97.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ft97_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_beta1: f64,
    param_beta2: f64,
    param_beta0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = param_beta1 * sigma[ip];
        let t21 = t18 * t18;
        let t22 = 1.0 / t21;
        let t23 = t20 * t22;
        let t24 = t11 * t11;
        let t25 = t11 * rho[ip];
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t28 = t24 * t27;
        let t29 = sigma[ip] * t22;
        let t32 = param_beta2 + t29 * t28 / 4.0;
        let t33 = 1.0 / t32;
        let t34 = t28 * t33;
        let t37 = param_beta0 + t23 * t34 / 4.0;
        let t38 = t37 * sigma[ip];
        let t39 = M_CBRT2;
        let t40 = t39 * t39;
        let t41 = rho[ip] * rho[ip];
        let t43 = 1.0 / t21 / t41;
        let t44 = t40 * t43;
        let t45 = t38 * t44;
        let t46 = t3 * t3;
        let t48 = pow_1_3(1.0 / M_PI);
        let t49 = 1.0 / t48;
        let t50 = t46 * t49;
        let t51 = M_CBRT4;
        let t52 = sigma[ip] * t40;
        let t53 = t37 * t37;
        let t55 = t52 * t43;
        let t56 = rmath::ln(t55 + rmath::sqrt(t55 * t55 + 1.0));
        let t57 = t56 * t56;
        let t61 = 9.0 * t52 * t43 * t53 * t57 + 1.0;
        let t62 = rmath::sqrt(t61);
        let t65 = t50 * t51 / t62;
        let t68 = 1.0 + 2.0 / 9.0 * t45 * t65;
        let t72 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t68);
        let tzk0 = 2.0 * t72;
        zk[ip] += tzk0;
        let t73 = t17 * t22;
        let t78 = 1.0 / t21 / rho[ip];
        let t79 = t20 * t78;
        let t82 = t24 * t11;
        let t83 = 1.0 / t26;
        let t84 = t82 * t83;
        let t85 = t84 * t33;
        let t88 = t32 * t32;
        let t89 = 1.0 / t88;
        let t90 = sigma[ip] * t78;
        let t94 = -t90 * t28 / 6.0 + t29 * t84 / 6.0;
        let t95 = t89 * t94;
        let t96 = t28 * t95;
        let t99 = -t79 * t34 / 6.0 + t23 * t85 / 6.0 - t23 * t96 / 4.0;
        let t100 = t99 * sigma[ip];
        let t101 = t100 * t44;
        let t104 = t41 * rho[ip];
        let t106 = 1.0 / t21 / t104;
        let t107 = t40 * t106;
        let t108 = t38 * t107;
        let t112 = 1.0 / t62 / t61;
        let t113 = t51 * t112;
        let t118 = t37 * t57;
        let t119 = t118 * t99;
        let t122 = sigma[ip] * sigma[ip];
        let t123 = t122 * t39;
        let t124 = t41 * t41;
        let t125 = t124 * t41;
        let t127 = 1.0 / t18 / t125;
        let t128 = t123 * t127;
        let t129 = t53 * t56;
        let t130 = t124 * rho[ip];
        let t132 = 1.0 / t18 / t130;
        let t135 = 2.0 * t123 * t132 + 1.0;
        let t136 = rmath::sqrt(t135);
        let t137 = 1.0 / t136;
        let t138 = t129 * t137;
        let t141 = -24.0 * t52 * t106 * t53 * t57 + 18.0 * t55 * t119 - 96.0 * t128 * t138;
        let t143 = t50 * t113 * t141;
        let t146 = 2.0 / 9.0 * t101 * t65 - 16.0 / 27.0 * t108 * t65 - t45 * t143 / 9.0;
        let t151 = piecewise3(t2, 0.0, -t6 * t73 * t68 / 8.0 - 3.0 / 8.0 * t6 * t19 * t146);
        let tvrho0 = 2.0 * rho[ip] * t151 + 2.0 * t72;
        vrho[ip] += tvrho0;
        let t154 = param_beta1 * t22;
        let t158 = 1.0 / t18 / rho[ip];
        let t159 = t20 * t158;
        let t160 = t24 * t24;
        let t161 = t26 * t25;
        let t162 = t160 * t161;
        let t163 = t162 * t89;
        let t166 = t154 * t34 / 4.0 - t159 * t163 / 16.0;
        let t167 = t166 * sigma[ip];
        let t168 = t167 * t44;
        let t171 = t37 * t40;
        let t175 = t53 * t57;
        let t178 = t118 * t166;
        let t181 = sigma[ip] * t39;
        let t182 = t181 * t132;
        let t185 = 36.0 * t182 * t138 + 9.0 * t44 * t175 + 18.0 * t55 * t178;
        let t187 = t50 * t113 * t185;
        let t190 = 2.0 / 9.0 * t168 * t65 + 2.0 / 9.0 * t171 * t43 * t65 - t45 * t187 / 9.0;
        let t194 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t190);
        let tvsigma0 = 2.0 * rho[ip] * t194;
        vsigma[ip] += tvsigma0;
    }
}
