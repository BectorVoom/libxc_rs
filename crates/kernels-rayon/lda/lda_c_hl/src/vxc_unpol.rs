//! LDA_C_HL vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_hl.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};
use libxc_rkernel_math::piecewise::{piecewise3};

/// LDA_C_HL vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_hl_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t4 = t2 / rho[ip];
        let t5 = param_hl_r_0;
        let t6 = t5 * t5;
        let t7 = t6 * t5;
        let t8 = 1.0 / t7;
        let t11 = 1.0 + 3.0 / 4.0 * t4 * t8;
        let t12 = M_CBRT3;
        let t13 = t12 * t12;
        let t14 = pow_1_3(t2);
        let t15 = 1.0 / t14;
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t23 = 1.0 + t16 * t19 * t5 / 3.0;
        let t24 = f64::ln(t23);
        let t26 = t14 * t14;
        let t27 = t13 * t26;
        let t28 = t18 * t18;
        let t30 = t17 / t28;
        let t31 = 1.0 / t6;
        let t35 = t12 * t14;
        let t36 = t17 * t17;
        let t38 = t36 / t18;
        let t39 = 1.0 / t5;
        let t44 = t1 * (t11 * t24 - t27 * t30 * t31 / 4.0 + t35 * t38 * t39 / 8.0 - 1.0 / 3.0);
        let t46 = pow_1_3(zeta_threshold);
        let t48 = piecewise3(1.0 <= zeta_threshold, t46 * zeta_threshold, 1.0);
        let t51 = M_CBRT2;
        let t55 = (2.0 * t48 - 2.0) / (2.0 * t51 - 2.0);
        let t56 = param_hl_c_1;
        let t57 = param_hl_r_1;
        let t58 = t57 * t57;
        let t59 = t58 * t57;
        let t60 = 1.0 / t59;
        let t63 = 1.0 + 3.0 / 4.0 * t4 * t60;
        let t67 = 1.0 + t16 * t19 * t57 / 3.0;
        let t68 = f64::ln(t67);
        let t70 = 1.0 / t58;
        let t74 = 1.0 / t57;
        let t81 = t55 * (-t56 * (t63 * t68 - t27 * t30 * t70 / 4.0 + t35 * t38 * t74 / 8.0 - 1.0 / 3.0) + t44);
        let tzk0 = -t44 + t81;
        zk[ip] += tzk0;
        let t82 = rho[ip] * rho[ip];
        let t83 = 1.0 / t82;
        let t84 = t2 * t83;
        let t85 = t8 * t24;
        let t89 = t11 * t13 * t15;
        let t90 = 1.0 / t23;
        let t91 = t5 * t90;
        let t97 = t17 / t28 / rho[ip];
        let t103 = t36 / t18 / rho[ip];
        let t108 = t1 * (-3.0 / 4.0 * t84 * t85 + t89 * t30 * t91 / 9.0 + t27 * t97 * t31 / 6.0 - t35 * t103 * t39 / 24.0);
        let t109 = t60 * t68;
        let t113 = t63 * t13 * t15;
        let t114 = 1.0 / t67;
        let t115 = t57 * t114;
        let t128 = t55 * (-t56 * (-3.0 / 4.0 * t84 * t109 + t113 * t30 * t115 / 9.0 + t27 * t97 * t70 / 6.0 - t35 * t103 * t74 / 24.0) + t108);
        let tvrho0 = -t44 + t81 + rho[ip] * (-t108 + t128);
        vrho[ip] += tvrho0;
    }
}
