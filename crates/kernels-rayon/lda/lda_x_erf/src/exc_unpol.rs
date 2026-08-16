//! LDA_X_ERF exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_erf.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::erf::{erf_approx};

/// LDA_X_ERF exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_x_erf_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = t1 * t3 * t6;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t10 = 1.0 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t13 = piecewise3(t10, t11 * zeta_threshold, 1.0);
        let t14 = t9 * t13;
        let t15 = pow_1_3(rho[ip]);
        let t16 = pow_1_3(9.0);
        let t17 = t16 * t16;
        let t18 = t3 * t3;
        let t20 = t17 * t18 * param_hyb_omega_0;
        let t23 = piecewise3(t10, t11, 1.0);
        let t24 = 1.0 / t23;
        let t27 = t20 * t1 / t15 * t24 / 18.0;
        let t28 = 1.35 <= t27;
        let t29 = 1.35 < t27;
        let t30 = piecewise3(t29, t27, 1.35);
        let t31 = t30 * t30;
        let t34 = t31 * t31;
        let t35 = 1.0 / t34;
        let t37 = t34 * t31;
        let t38 = 1.0 / t37;
        let t40 = t34 * t34;
        let t41 = 1.0 / t40;
        let t44 = 1.0 / t40 / t31;
        let t47 = 1.0 / t40 / t34;
        let t50 = 1.0 / t40 / t37;
        let t52 = t40 * t40;
        let t53 = 1.0 / t52;
        let t56 = piecewise3(t29, 1.35, t27);
        let t57 = f64::sqrt(M_PI);
        let t58 = 1.0 / t56;
        let t60 = erf_approx(t58 / 2.0);
        let t62 = t56 * t56;
        let t63 = 1.0 / t62;
        let t65 = f64::exp(-t63 / 4.0);
        let t66 = t65 - 1.0;
        let t69 = t65 - 3.0 / 2.0 - 2.0 * t62 * t66;
        let t72 = 2.0 * t56 * t69 + t57 * t60;
        let t76 = piecewise3(t28, 1.0 / t31 / 36.0 - t35 / 960.0 + t38 / 26880.0 - t41 / 829440.0 + t44 / 28385280.0 - t47 / 1073479680.0 + t50 / 44590694400.0 - t53 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t56 * t72);
        let t79 = t7 * t14 * t15 * t76;
        let tzk0 = -3.0 / 16.0 * t79;
        zk[ip] += tzk0;
    }
}
