//! LDA_X_REL exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_rel.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};
use libxc_rkernel_math::piecewise::{piecewise3};

/// LDA_X_REL exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_x_rel_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t8 = pow_1_3(zeta_threshold);
        let t10 = piecewise3(1.0 <= zeta_threshold, t8 * zeta_threshold, 1.0);
        let t11 = pow_1_3(rho[ip]);
        let t15 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t10 * t11);
        let t16 = pow_1_3(9.0);
        let t17 = t16 * t16;
        let t18 = t17 * t3;
        let t19 = 1.0 / M_PI;
        let t20 = pow_1_3(t19);
        let t21 = t20 * t20;
        let t22 = 1.0 / t21;
        let t23 = t11 * t11;
        let t27 = 1.0 + 3.8075239991386495e-05 * t18 * t22 * t23;
        let t28 = f64::sqrt(t27);
        let t29 = t28 * t17;
        let t30 = t3 * t20;
        let t35 = t3 * t3;
        let t36 = t16 * t35;
        let t37 = 1.0 / t20;
        let t41 = f64::ln(0.0035625477770544352 * t36 * t37 * t11 + f64::sqrt(pow_2(0.0035625477770544352 * t36 * t37 * t11) + 1.0));
        let t42 = t41 * t16;
        let t43 = t35 * t21;
        let t44 = 1.0 / t23;
        let t48 = 10.396221848752237 * t29 * t30 / t11 - 972.7328585562606 * t42 * t43 * t44;
        let t49 = t48 * t48;
        let t51 = 1.0 - 1.5 * t49;
        let tzk0 = 2.0 * t15 * t51;
        zk[ip] += tzk0;
    }
}
