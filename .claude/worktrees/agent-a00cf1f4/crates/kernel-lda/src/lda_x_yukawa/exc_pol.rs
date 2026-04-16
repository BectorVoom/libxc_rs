//! LDA_X_YUKAWA exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 122 shared lines across all orders.
//! Delta: 122 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X_YUKAWA exc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_x_yukawa_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (122 lines) ---
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = t6 * t3 * t1;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t10 = rho0 - rho1;
        let t11 = rho0 + rho1;
        let t12 = 1.0 / t11;
        let t13 = t12 * t10;
        let t14 = 1.0 + t13;
        let t15 = t14 <= zeta_threshold;
        let t16 = pow_1_3(zeta_threshold);
        let t17 = t16 * zeta_threshold;
        let t18 = pow_1_3(t14);
        let t20 = piecewise3(t15, t17, t18 * t14);
        let t21 = t20 * t9;
        let t22 = pow_1_3(t11);
        let t23 = pow_1_3(9.0);
        let t24 = t23 * t23;
        let t25 = t3 * t3;
        let t26 = t25 * t24;
        let t27 = param_hyb_omega_0 * t26;
        let t28 = 1.0 / t22;
        let t29 = t28 * t1;
        let t30 = piecewise3(t15, t16, t18);
        let t31 = 1.0 / t30;
        let t34 = t31 * t29 * t27 / 18.0;
        let t35 = 1.92 <= t34;
        let t36 = 1.92 < t34;
        let t37 = piecewise3(t36, t34, 1.92);
        let t38 = t37 * t37;
        let t41 = t38 * t38;
        let t42 = 1.0 / t41;
        let t44 = t41 * t38;
        let t45 = 1.0 / t44;
        let t47 = t41 * t41;
        let t48 = 1.0 / t47;
        let t50 = t47 * t38;
        let t51 = 1.0 / t50;
        let t53 = t47 * t41;
        let t54 = 1.0 / t53;
        let t56 = t47 * t44;
        let t57 = 1.0 / t56;
        let t59 = t47 * t47;
        let t60 = 1.0 / t59;
        let t63 = 1.0 / t59 / t38;
        let t66 = 1.0 / t59 / t41;
        let t69 = 1.0 / t59 / t44;
        let t72 = 1.0 / t59 / t47;
        let t75 = 1.0 / t59 / t50;
        let t78 = 1.0 / t59 / t53;
        let t81 = 1.0 / t59 / t56;
        let t83 = t59 * t59;
        let t84 = 1.0 / t83;
        let t87 = 1.0 / t83 / t38;
        let t90 = 1.0 / t83 / t41;
        let t92 = 1.0 / t38 / 9.0 - t42 / 30.0 + t45 / 70.0 - t48 / 135.0 + t51 / 231.0 - t54 / 364.0 + t57 / 540.0 - t60 / 765.0 + t63 / 1045.0 - t66 / 1386.0 + t69 / 1794.0 - t72 / 2275.0 + t75 / 2835.0 - t78 / 3480.0 + t81 / 4216.0 - t84 / 5049.0 + t87 / 5985.0 - t90 / 7030.0;
        let t93 = piecewise3(t36, 1.92, t34);
        let t94 = f64::atan2(1.0, t93);
        let t95 = t93 * t93;
        let t96 = t95 + 3.0;
        let t97 = 1.0 / t95;
        let t98 = 1.0 + t97;
        let t99 = f64::ln(t98);
        let t101 = -t99 * t96 + 1.0;
        let t104 = t94 + t101 * t93 / 4.0;
        let t108 = piecewise3(t35, t92, 1.0 - 8.0 / 3.0 * t104 * t93);
        let t109 = t108 * t22;
        let t111 = t109 * t21 * t7;
        let t112 = 1.0 - t13;
        let t113 = t112 <= zeta_threshold;
        let t114 = pow_1_3(t112);
        let t116 = piecewise3(t113, t17, t114 * t112);
        let t117 = t116 * t9;
        let t118 = piecewise3(t113, t16, t114);
        let t119 = 1.0 / t118;
        let t122 = t119 * t29 * t27 / 18.0;
        let t123 = 1.92 <= t122;
        let t124 = 1.92 < t122;
        let t125 = piecewise3(t124, t122, 1.92);
        let t126 = t125 * t125;
        let t129 = t126 * t126;
        let t130 = 1.0 / t129;
        let t132 = t129 * t126;
        let t133 = 1.0 / t132;
        let t135 = t129 * t129;
        let t136 = 1.0 / t135;
        let t138 = t135 * t126;
        let t139 = 1.0 / t138;
        let t141 = t135 * t129;
        let t142 = 1.0 / t141;
        let t144 = t135 * t132;
        let t145 = 1.0 / t144;
        let t147 = t135 * t135;
        let t148 = 1.0 / t147;
        let t151 = 1.0 / t147 / t126;
        let t154 = 1.0 / t147 / t129;
        let t157 = 1.0 / t147 / t132;
        let t160 = 1.0 / t147 / t135;
        let t163 = 1.0 / t147 / t138;
        let t166 = 1.0 / t147 / t141;
        let t169 = 1.0 / t147 / t144;
        let t171 = t147 * t147;
        let t172 = 1.0 / t171;
        let t175 = 1.0 / t171 / t126;
        let t178 = 1.0 / t171 / t129;
        let t180 = 1.0 / t126 / 9.0 - t130 / 30.0 + t133 / 70.0 - t136 / 135.0 + t139 / 231.0 - t142 / 364.0 + t145 / 540.0 - t148 / 765.0 + t151 / 1045.0 - t154 / 1386.0 + t157 / 1794.0 - t160 / 2275.0 + t163 / 2835.0 - t166 / 3480.0 + t169 / 4216.0 - t172 / 5049.0 + t175 / 5985.0 - t178 / 7030.0;
        let t181 = piecewise3(t124, 1.92, t122);
        let t182 = f64::atan2(1.0, t181);
        let t183 = t181 * t181;
        let t184 = t183 + 3.0;
        let t185 = 1.0 / t183;
        let t186 = 1.0 + t185;
        let t187 = f64::ln(t186);
        let t189 = -t187 * t184 + 1.0;
        let t192 = t182 + t189 * t181 / 4.0;
        let t196 = piecewise3(t123, t180, 1.0 - 8.0 / 3.0 * t192 * t181);
        let t197 = t196 * t22;
        let t199 = t197 * t117 * t7;
        let tzk0 = -3.0 / 32.0 * t111 - 3.0 / 32.0 * t199;
        zk[ip] += tzk0;
    }
}
