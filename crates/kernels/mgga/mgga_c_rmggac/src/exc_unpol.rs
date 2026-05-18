//! MGGA_C_RMGGAC exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rmggac.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_rmggac_exc_unpol(
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
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3::<f64>(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = pow_1_3::<f64>(rho[ip]);
        let t11 = t5 * t7 / t8;
        let t12 = f64::sqrt(t11);
        let t15 = 1.0 + 0.4445e-1 * t12 + 0.3138525e-1 * t11;
        let t16 = 1.0 / t15;
        let t19 = f64::exp(1.0 * t16);
        let t20 = t19 - 1.0;
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3::<f64>(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = t28 * sigma[ip];
        let t30 = rho[ip] * rho[ip];
        let t31 = t8 * t8;
        let t33 = 1.0 / t31 / t30;
        let t35 = t26 * t29 * t33;
        let t37 = 1.0 + 0.21337642104376358333e-1 * t35;
        let t38 = pow_1_4::<f64>(t37);
        let t40 = 1.0 - 1.0 / t38;
        let t42 = t20 * t40 + 1.0;
        let t43 = f64::ln(t42);
        let t46 = t27 - 1.0;
        let t47 = 1.0 <= zeta_threshold;
        let t48 = pow_1_3::<f64>(zeta_threshold);
        let t50 = piecewise3::<f64>(t47, t48 * zeta_threshold, 1.0);
        let t52 = 2.0 * t50 - 2.0;
        let t55 = 1.0 / t46 / 2.0;
        let t58 = 1.0 - 0.2363e1 * t46 * t52 * t55;
        let t59 = (-0.285764e-1 * t16 + 0.285764e-1 * t43) * t58;
        let t61 = 1.0 / t31 / rho[ip];
        let t66 = 2.0 * tau[ip] * t61 - sigma[ip] * t33 / 4.0;
        let t67 = t66 * t66;
        let t68 = t67 * t66;
        let t73 = 0.8e-1 + 5.0 / 18.0 * t66 * t28 * t26 + 0.125e-1 * t35;
        let t74 = t73 * t73;
        let t75 = t74 * t73;
        let t76 = 1.0 / t75;
        let t77 = t68 * t76;
        let t79 = t67 * t67;
        let t80 = t79 * t67;
        let t81 = t74 * t74;
        let t83 = 1.0 / t81 / t74;
        let t86 = 1.0 + 0.66523565010354492023e-2 * t77 + 0.44253847016868604463e-4 * t80 * t83;
        let t87 = 1.0 / t86;
        let t88 = t77 * t87;
        let t90 = 1.0 - 0.19957069503106347607e-1 * t88;
        let t91 = t59 * t90;
        let t93 = 1.0 + 0.53425e-1 * t11;
        let t96 = pow_3_2::<f64>(t11);
        let t98 = t2 * t2;
        let t99 = t4 * t4;
        let t100 = t98 * t99;
        let t103 = t100 * t6 / t31;
        let t105 = 0.379785e1 * t12 + 0.8969e0 * t11 + 0.204775e0 * t96 + 0.123235e0 * t103;
        let t108 = 1.0 + 0.16081979498692535067e2 / t105;
        let t109 = f64::ln(t108);
        let t111 = 0.621814e-1 * t93 * t109;
        let t112 = t52 * t55;
        let t114 = 1.0 + 0.278125e-1 * t11;
        let t119 = 0.51785e1 * t12 + 0.905775e0 * t11 + 0.1100325e0 * t96 + 0.1241775e0 * t103;
        let t122 = 1.0 + 0.29608749977793437516e2 / t119;
        let t123 = f64::ln(t122);
        let t126 = 0.19751673498613801407e-1 * t112 * t114 * t123;
        let t127 = t48 * t48;
        let t128 = piecewise3::<f64>(t47, t127, 1.0);
        let t129 = t128 * t128;
        let t130 = t129 * t128;
        let t131 = -t111 + t126;
        let t132 = 1.0 / t130;
        let t135 = f64::exp(-0.32163648644302209643e2 * t131 * t132);
        let t136 = t135 - 1.0;
        let t137 = f64::ln(2.0);
        let t138 = 1.0 - t137;
        let t139 = 1.0 / t138;
        let t143 = f64::exp(-t131 * t139 * t22 * t132);
        let t144 = t143 - 1.0;
        let t145 = 1.0 / t144;
        let t146 = t139 * t145;
        let t148 = 1.0 / t8 / t30;
        let t151 = 1.0 / t129;
        let t153 = 1.0 / t4;
        let t154 = t98 * t153;
        let t155 = t154 * t6;
        let t156 = t27 * t151 * t155;
        let t159 = 1.0 + 0.27439556402611977244e-1 * t146 * sigma[ip] * t148 * t156;
        let t160 = pow_1_4::<f64>(t159);
        let t162 = 1.0 - 1.0 / t160;
        let t164 = t136 * t162 + 1.0;
        let t165 = f64::ln(t164);
        let t168 = -t111 + t126 + 0.31091e-1 * t130 * t165;
        let t169 = t168 * t68;
        let t170 = t76 * t87;
        let t172 = 0.19957069503106347607e-1 * t169 * t170;
        let tzk0 = t91 + t172;
        zk[ip] += tzk0;
    }
}
