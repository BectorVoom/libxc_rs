//! LDA_C_VWN_1 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 38 shared lines across all orders.
//! Delta: 49 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_VWN_1 vxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_vwn_1_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (38 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = f64::sqrt(t10);
        let t14 = t11 + 1.86372 * t12 + 12.9352;
        let t15 = 1.0 / t14;
        let t19 = f64::ln(t4 * t9 * t15 / 4.0);
        let t21 = t12 + 3.72744;
        let t24 = f64::atan(6.15199081975908 / t21);
        let t26 = t12 / 2.0;
        let t27 = t26 + 0.10498;
        let t28 = t27 * t27;
        let t30 = f64::ln(t28 * t15);
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(1.0 <= zeta_threshold, t34 * zeta_threshold, 1.0);
        let t38 = 2.0 * t36 - 2.0;
        let t39 = M_CBRT2;
        let t42 = 1.0 / (2.0 * t39 - 2.0);
        let t44 = -t38 * t42 + 1.0;
        let t45 = (0.0310907 * t19 + 0.038783294878113016 * t24 + 0.0009690227711544374 * t30) * t44;
        let t47 = t11 + 3.53021 * t12 + 18.0578;
        let t48 = 1.0 / t47;
        let t52 = f64::ln(t4 * t9 * t48 / 4.0);
        let t54 = t12 + 7.06042;
        let t57 = f64::atan(4.730926909560113 / t54);
        let t59 = t26 + 0.325;
        let t60 = t59 * t59;
        let t62 = f64::ln(t60 * t48);
        let t66 = (0.01554535 * t52 + 0.05249139316978094 * t57 + 0.0022478670955426118 * t62) * t38 * t42;
        let tzk0 = t45 + t66;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (49 lines) ---
        let t68 = 1.0 / t7 / rho[ip];
        let t69 = t6 * t68;
        let t73 = t4 * t6;
        let t74 = t14 * t14;
        let t75 = 1.0 / t74;
        let t76 = t8 * t75;
        let t77 = t4 * t69;
        let t78 = t77 / 12.0;
        let t79 = 1.0 / t12;
        let t80 = t79 * t1;
        let t81 = t3 * t6;
        let t83 = t80 * t81 * t68;
        let t85 = -t78 - 0.31062 * t83;
        let t90 = t1 * t1;
        let t92 = 1.0 / t3;
        let t93 = (-t4 * t69 * t15 / 12.0 - t73 * t76 * t85 / 4.0) * t90 * t92;
        let t94 = t5 * t7;
        let t95 = t94 * t14;
        let t98 = t21 * t21;
        let t99 = 1.0 / t98;
        let t101 = t99 * t79 * t1;
        let t103 = 37.8469910464 * t99 + 1.0;
        let t104 = 1.0 / t103;
        let t109 = t27 * t15;
        let t110 = t109 * t79;
        let t113 = t28 * t75;
        let t115 = -t110 * t77 / 6.0 - t113 * t85;
        let t116 = 1.0 / t28;
        let t117 = t115 * t116;
        let t121 = (0.010363566666666667 * t93 * t95 + 0.03976574567502677 * t101 * t81 * t68 * t104 + 0.0009690227711544374 * t117 * t14) * t44;
        let t125 = t47 * t47;
        let t126 = 1.0 / t125;
        let t127 = t8 * t126;
        let t129 = -t78 - 0.5883683333333334 * t83;
        let t135 = (-t4 * t69 * t48 / 12.0 - t73 * t127 * t129 / 4.0) * t90 * t92;
        let t136 = t94 * t47;
        let t139 = t54 * t54;
        let t140 = 1.0 / t139;
        let t142 = t140 * t79 * t1;
        let t144 = 22.3816694236 * t140 + 1.0;
        let t145 = 1.0 / t144;
        let t150 = t59 * t48;
        let t151 = t150 * t79;
        let t154 = t60 * t126;
        let t156 = -t151 * t77 / 6.0 - t154 * t129;
        let t157 = 1.0 / t60;
        let t158 = t156 * t157;
        let t163 = (0.005181783333333334 * t135 * t136 + 0.041388824077869424 * t142 * t81 * t68 * t145 + 0.0022478670955426118 * t158 * t47) * t38 * t42;
        let tvrho0 = t45 + t66 + rho[ip] * (t121 + t163);
        vrho[ip] += tvrho0;
    }
}
