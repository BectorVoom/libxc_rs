//! LDA_X_REL vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_rel.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X_REL vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_x_rel_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t8 = rho0 * t7;
        let t10 = 2.0 * t8 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t12 = t11 * zeta_threshold;
        let t13 = M_CBRT2;
        let t14 = t13 * rho0;
        let t15 = pow_1_3(t8);
        let t19 = piecewise3(t10, t12, 2.0 * t14 * t7 * t15);
        let t20 = pow_1_3(t6);
        let t24 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t19 * t20);
        let t25 = rho1 <= dens_threshold;
        let t26 = rho1 * t7;
        let t28 = 2.0 * t26 <= zeta_threshold;
        let t29 = t13 * rho1;
        let t30 = pow_1_3(t26);
        let t34 = piecewise3(t28, t12, 2.0 * t29 * t7 * t30);
        let t38 = piecewise3(t25, 0.0, -3.0 / 8.0 * t5 * t34 * t20);
        let t39 = t24 + t38;
        let t40 = pow_1_3(9.0);
        let t41 = t40 * t40;
        let t42 = t41 * t2;
        let t43 = 1.0 / M_PI;
        let t44 = pow_1_3(t43);
        let t45 = t44 * t44;
        let t46 = 1.0 / t45;
        let t47 = t20 * t20;
        let t51 = 1.0 + 3.8075239991386495e-05 * t42 * t46 * t47;
        let t52 = f64::sqrt(t51);
        let t53 = t52 * t41;
        let t54 = t2 * t44;
        let t59 = t2 * t2;
        let t60 = t40 * t59;
        let t61 = 1.0 / t44;
        let t65 = f64::ln(0.0035625477770544352 * t60 * t61 * t20 + f64::sqrt(pow_2(0.0035625477770544352 * t60 * t61 * t20) + 1.0));
        let t66 = t65 * t40;
        let t67 = t59 * t45;
        let t68 = 1.0 / t47;
        let t72 = 10.396221848752237 * t53 * t54 / t20 - 972.7328585562606 * t66 * t67 * t68;
        let t73 = t72 * t72;
        let t75 = 1.0 - 1.5 * t73;
        let tzk0 = t39 * t75;
        zk[ip] += tzk0;
        let t76 = t13 * t7;
        let t79 = t6 * t6;
        let t80 = 1.0 / t79;
        let t83 = 2.0 * t14 * t80 * t15;
        let t84 = t15 * t15;
        let t85 = 1.0 / t84;
        let t86 = t7 * t85;
        let t88 = -rho0 * t80 + t7;
        let t93 = piecewise3(t10, 0.0, 2.0 * t76 * t15 - t83 + 2.0 / 3.0 * t14 * t86 * t88);
        let t99 = t5 * t19 * t68 / 8.0;
        let t101 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t93 * t20 - t99);
        let t104 = 2.0 * t29 * t80 * t30;
        let t105 = rho1 * rho1;
        let t106 = t13 * t105;
        let t107 = t79 * t6;
        let t108 = 1.0 / t107;
        let t109 = t30 * t30;
        let t110 = 1.0 / t109;
        let t111 = t108 * t110;
        let t115 = piecewise3(t28, 0.0, -t104 - 2.0 / 3.0 * t106 * t111);
        let t121 = t5 * t34 * t68 / 8.0;
        let t123 = piecewise3(t25, 0.0, -3.0 / 8.0 * t5 * t115 * t20 - t121);
        let t124 = t101 + t123;
        let t125 = t6 * t124;
        let t127 = t6 * t39;
        let t128 = 1.0 / t52;
        let t129 = t128 * t40;
        let t130 = t59 * t61;
        let t135 = 1.0 / t20 / t6;
        let t136 = t54 * t135;
        let t139 = t128 * t41;
        let t143 = 1.0 / t47 / t6;
        let t147 = 0.0011875159256848119 * t129 * t130 * t68 - 3.4654072829174125 * t53 * t136 - 3.4654072829174125 * t139 * t136 + 648.4885723708404 * t66 * t67 * t143;
        let t148 = t72 * t147;
        let t150 = 3.0 * t127 * t148;
        let tvrho0 = t125 * t75 - t150 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t151 = rho0 * rho0;
        let t152 = t13 * t151;
        let t153 = t108 * t85;
        let t157 = piecewise3(t10, 0.0, -t83 - 2.0 / 3.0 * t152 * t153);
        let t162 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t157 * t20 - t99);
        let t165 = t7 * t110;
        let t167 = -rho1 * t80 + t7;
        let t172 = piecewise3(t28, 0.0, 2.0 * t76 * t30 - t104 + 2.0 / 3.0 * t29 * t165 * t167);
        let t177 = piecewise3(t25, 0.0, -3.0 / 8.0 * t5 * t172 * t20 - t121);
        let t178 = t162 + t177;
        let t179 = t6 * t178;
        let tvrho1 = t179 * t75 - t150 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
