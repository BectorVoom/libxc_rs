//! LDA_C_W20 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_w20.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_W20 exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_w20_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = f64::ln(2.0);
        let t2 = 1.0 - t1;
        let t3 = M_PI * M_PI;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = t1 / 6.0;
        let t8 = 1.0 / t2;
        let t12 = f64::exp(-2.0 * (-0.16244537117517982 + t6) * t8 * t3);
        let t13 = M_CBRT3;
        let t14 = t13 * t13;
        let t15 = 1.0 / M_PI;
        let t16 = pow_1_3(t15);
        let t17 = t16 * t16;
        let t18 = t14 * t17;
        let t19 = M_CBRT4;
        let t20 = pow_1_3(rho[ip]);
        let t21 = t20 * t20;
        let t22 = 1.0 / t21;
        let t24 = t18 * t19 * t22;
        let t26 = f64::exp(-t24 / 40000.0);
        let t27 = 1.0 - t26;
        let t28 = M_CBRTPI;
        let t29 = t28 * t28;
        let t31 = pow_1_3(9.0);
        let t32 = 1.0 / t29 * t31;
        let t33 = t19 * t19;
        let t39 = t12 / 2.0;
        let t40 = (-0.9 + 3.0 / 16.0 * t32 * t33) * t8 * t3 + t39;
        let t44 = (-2.0 * t27 * t40 + t12) * t14;
        let t45 = 1.0 / t16;
        let t46 = t45 * t19;
        let t47 = t46 * t20;
        let t50 = t27 * t8;
        let t51 = f64::sqrt(4.0);
        let t52 = t13 * t16;
        let t53 = 1.0 / t20;
        let t55 = t52 * t33 * t53;
        let t56 = f64::sqrt(t55);
        let t58 = 1.0 / t56 / t55;
        let t60 = t50 * t51 * t58;
        let t62 = t31 * t31;
        let t63 = t62 * t19;
        let t64 = t29 * t3;
        let t68 = -3.0 / 40.0 * t63 * t64 * t8 + t39;
        let t72 = (-2.0 * t27 * t68 + t12) * t13;
        let t73 = 1.0 / t17;
        let t74 = t73 * t33;
        let t75 = t74 * t21;
        let t78 = 1.0 + t44 * t47 / 3.0 - 118.43525281307231 * t60 + t72 * t75 / 3.0;
        let t79 = f64::ln(t78);
        let t81 = t5 * t79 / 2.0;
        let t82 = t52 * t33;
        let t83 = t53 * t26;
        let t84 = pow_1_4(4.0);
        let t85 = t84 * t84;
        let t86 = t85 * t84;
        let t87 = pow_1_4(t55);
        let t91 = t26 + 5.0 / 8.0 * t86 * t87 * t55;
        let t92 = 1.0 / t91;
        let t93 = t3 * M_PI;
        let t95 = 1.0 / t28 / t93;
        let t97 = 12.0 * t1;
        let t98 = 7.0 / 6.0 * t3 - t97 - 1.0;
        let t99 = t95 * t98;
        let t100 = t14 * t45;
        let t104 = 1.0 + t100 * t19 * t20 / 3.0;
        let t105 = f64::ln(t104);
        let t109 = -t63 * t99 * t105 / 36.0 - 0.01;
        let t110 = t92 * t109;
        let t113 = t82 * t83 * t110 / 4.0;
        let t118 = f64::exp(-4.0 * (-0.1412623711751798 + t6) * t8 * t3);
        let t119 = M_CBRT2;
        let t127 = t118 / 2.0;
        let t128 = 2.0 * (-0.9 + 3.0 / 16.0 * t32 * t33 * t119) * t8 * t3 + t127;
        let t132 = (-2.0 * t27 * t128 + t118) * t14;
        let t136 = t119 * t119;
        let t141 = -3.0 / 20.0 * t63 * t64 * t136 * t8 + t127;
        let t145 = (-2.0 * t27 * t141 + t118) * t13;
        let t148 = 1.0 + t132 * t47 / 3.0 - 236.87050562614462 * t60 + t145 * t75 / 3.0;
        let t149 = f64::ln(t148);
        let t154 = t136 * t62;
        let t156 = 13.0 / 12.0 * t3 - t97 + 1.0 / 2.0;
        let t157 = t95 * t156;
        let t159 = t154 * t157 * t105;
        let t164 = pow_1_3(zeta_threshold);
        let t166 = piecewise3(1.0 <= zeta_threshold, t164 * zeta_threshold, 1.0);
        let t168 = 2.0 * t166 - 2.0;
        let t172 = 1.0 / (2.0 * t119 - 2.0);
        let t173 = (-t5 * t149 / 4.0 - t52 * t83 * t92 * t159 / 144.0 + t81 - t113) * t168 * t172;
        let tzk0 = -t81 + t113 + t173;
        zk[ip] += tzk0;
    }
}
