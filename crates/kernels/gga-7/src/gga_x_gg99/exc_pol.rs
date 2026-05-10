//! GGA_X_GG99 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 155 shared lines across all orders.
//! Delta: 155 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_2};
use libxc_kernel_math::special::{xc_dilogarithm};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_gg99_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
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
        // --- shared preamble (155 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t7 = t3 / t4 / M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t21);
        let t27 = piecewise3(t22, t24, t25 * t21);
        let t28 = pow_1_3(t8);
        let t29 = t27 * t28;
        let t30 = t7 * t29;
        let t31 = M_PI * M_PI;
        let t32 = f64::sqrt(sigma0);
        let t33 = pow_1_3(rho0);
        let t35 = 1.0 / t33 / rho0;
        let t36 = t32 * t35;
        let t37 = M_CBRT4;
        let t38 = f64::sqrt(3.0);
        let t39 = t31 * M_PI;
        let t40 = t38 * t39;
        let t41 = pow_1_3(t40);
        let t42 = t37 * t41;
        let t43 = t36 < t42;
        let t44 = pow_1_4(3.0);
        let t45 = M_SQRT2;
        let t46 = t44 * t45;
        let t47 = f64::sqrt(M_PI);
        let t49 = 1.0 / t47 / M_PI;
        let t50 = t46 * t49;
        let t51 = t42 - 0.1e-9;
        let t52 = t51 < t36;
        let t53 = piecewise3(t52, t51, t36);
        let t54 = t53 * t53;
        let t55 = 4.0 * t40;
        let t56 = t31 * t31;
        let t57 = t56 * t31;
        let t58 = 48.0 * t57;
        let t59 = t54 * t54;
        let t60 = t59 * t54;
        let t61 = t58 - t60;
        let t62 = f64::sqrt(t61);
        let t63 = t55 + t62;
        let t64 = pow_1_3(t63);
        let t65 = t64 * t64;
        let t66 = t54 + t65;
        let t67 = f64::sqrt(t66);
        let t69 = f64::powf(t63, 1.0 / 6.0);
        let t70 = 1.0 / t69;
        let t74 = f64::ln(t50 * t53 * t67 * t70 / 4.0 + f64::sqrt(pow_2(t50 * t53 * t67 * t70 / 4.0) + 1.0));
        let t75 = 1.0 / M_PI;
        let t76 = t42 + 0.1e-9;
        let t77 = t76 < t36;
        let t78 = piecewise3(t77, t36, t76);
        let t79 = t78 * t78;
        let t80 = t79 * t78;
        let t81 = t80 * t38;
        let t82 = 1.0 / t57;
        let t83 = t79 * t79;
        let t84 = t83 * t79;
        let t87 = 3.0 * t82 * t84 - 144.0;
        let t88 = f64::sqrt(t87);
        let t90 = f64::atan(t88 / 12.0);
        let t91 = t90 / 3.0;
        let t92 = f64::cos(t91);
        let t93 = t75 * t92;
        let t94 = t81 * t93;
        let t95 = f64::sqrt(t94);
        let t98 = f64::ln(t75 * t95 / 2.0 + f64::sqrt(pow_2(t75 * t95 / 2.0) + 1.0));
        let t99 = piecewise3(t43, t74, t98);
        let t101 = f64::exp(-2.0 * t99);
        let t102 = 1.0 + t101;
        let t103 = f64::ln(t102);
        let t106 = xc_dilogarithm(-t101);
        let t108 = -12.0 * t99 * t103 + 12.0 * t106 + t31;
        let t109 = 1.0 / t99;
        let t111 = 1.0 / f64::cosh(t99);
        let t112 = pow_1_3(t111);
        let t113 = t112 * t112;
        let t114 = 1.0 / t113;
        let t115 = pow_1_3(t75);
        let t116 = 1.0 / t115;
        let t118 = t114 * t116 * t37;
        let t119 = t108 * t109 * t118;
        let t122 = piecewise3(t1, 0.0, -t30 * t119 / 24.0);
        let t123 = rho1 <= dens_threshold;
        let t124 = -t18;
        let t126 = piecewise5(t16, t13, t12, t17, t124 * t9);
        let t127 = 1.0 + t126;
        let t128 = t127 <= zeta_threshold;
        let t129 = pow_1_3(t127);
        let t131 = piecewise3(t128, t24, t129 * t127);
        let t132 = t131 * t28;
        let t133 = t7 * t132;
        let t134 = f64::sqrt(sigma2);
        let t135 = pow_1_3(rho1);
        let t137 = 1.0 / t135 / rho1;
        let t138 = t134 * t137;
        let t139 = t138 < t42;
        let t140 = t51 < t138;
        let t141 = piecewise3(t140, t51, t138);
        let t142 = t141 * t141;
        let t143 = t142 * t142;
        let t144 = t143 * t142;
        let t145 = t58 - t144;
        let t146 = f64::sqrt(t145);
        let t147 = t55 + t146;
        let t148 = pow_1_3(t147);
        let t149 = t148 * t148;
        let t150 = t142 + t149;
        let t151 = f64::sqrt(t150);
        let t153 = f64::powf(t147, 1.0 / 6.0);
        let t154 = 1.0 / t153;
        let t158 = f64::ln(t50 * t141 * t151 * t154 / 4.0 + f64::sqrt(pow_2(t50 * t141 * t151 * t154 / 4.0) + 1.0));
        let t159 = t76 < t138;
        let t160 = piecewise3(t159, t138, t76);
        let t161 = t160 * t160;
        let t162 = t161 * t160;
        let t163 = t162 * t38;
        let t164 = t161 * t161;
        let t165 = t164 * t161;
        let t168 = 3.0 * t82 * t165 - 144.0;
        let t169 = f64::sqrt(t168);
        let t171 = f64::atan(t169 / 12.0);
        let t172 = t171 / 3.0;
        let t173 = f64::cos(t172);
        let t174 = t75 * t173;
        let t175 = t163 * t174;
        let t176 = f64::sqrt(t175);
        let t179 = f64::ln(t75 * t176 / 2.0 + f64::sqrt(pow_2(t75 * t176 / 2.0) + 1.0));
        let t180 = piecewise3(t139, t158, t179);
        let t182 = f64::exp(-2.0 * t180);
        let t183 = 1.0 + t182;
        let t184 = f64::ln(t183);
        let t187 = xc_dilogarithm(-t182);
        let t189 = -12.0 * t180 * t184 + 12.0 * t187 + t31;
        let t190 = 1.0 / t180;
        let t192 = 1.0 / f64::cosh(t180);
        let t193 = pow_1_3(t192);
        let t194 = t193 * t193;
        let t195 = 1.0 / t194;
        let t197 = t195 * t116 * t37;
        let t198 = t189 * t190 * t197;
        let t201 = piecewise3(t123, 0.0, -t133 * t198 / 24.0);
        let tzk0 = t122 + t201;
        zk[ip] += tzk0;
    }
}
