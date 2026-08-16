//! LDA_C_VWN_1 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_1.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};
use libxc_rkernel_math::piecewise::{piecewise3};

/// LDA_C_VWN_1 exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_1_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
    }
}
