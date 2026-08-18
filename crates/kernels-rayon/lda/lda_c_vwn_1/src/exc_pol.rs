//! LDA_C_VWN_1 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_1.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_1_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = f64::sqrt(t11);
        let t15 = t12 + 1.86372 * t13 + 12.9352;
        let t16 = 1.0 / t15;
        let t20 = f64::ln(t4 * t10 * t16 / 4.0);
        let t22 = t13 + 3.72744;
        let t25 = f64::atan(6.15199081975908 / t22);
        let t27 = t13 / 2.0;
        let t28 = t27 + 0.10498;
        let t29 = t28 * t28;
        let t31 = f64::ln(t29 * t16);
        let t33 = 0.0310907 * t20 + 0.038783294878113016 * t25 + 0.0009690227711544374 * t31;
        let t34 = rho0 - rho1;
        let t35 = 1.0 / t7;
        let t36 = t34 * t35;
        let t37 = 1.0 + t36;
        let t38 = t37 <= zeta_threshold;
        let t39 = pow_1_3(zeta_threshold);
        let t40 = t39 * zeta_threshold;
        let t41 = pow_1_3(t37);
        let t43 = piecewise3(t38, t40, t41 * t37);
        let t44 = 1.0 - t36;
        let t45 = t44 <= zeta_threshold;
        let t46 = pow_1_3(t44);
        let t48 = piecewise3(t45, t40, t46 * t44);
        let t49 = t43 + t48 - 2.0;
        let t50 = M_CBRT2;
        let t53 = 1.0 / (2.0 * t50 - 2.0);
        let t55 = -t49 * t53 + 1.0;
        let t56 = t33 * t55;
        let t58 = t12 + 3.53021 * t13 + 18.0578;
        let t59 = 1.0 / t58;
        let t63 = f64::ln(t4 * t10 * t59 / 4.0);
        let t65 = t13 + 7.06042;
        let t68 = f64::atan(4.730926909560113 / t65);
        let t70 = t27 + 0.325;
        let t71 = t70 * t70;
        let t73 = f64::ln(t71 * t59);
        let t75 = 0.01554535 * t63 + 0.05249139316978094 * t68 + 0.0022478670955426118 * t73;
        let t77 = t75 * t49 * t53;
        let tzk0 = t56 + t77;
        zk[ip] += tzk0;
    }
}
