//! LDA_C_1D_LOOS exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_1d_loos.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI, M_SQRT2};

/// LDA_C_1D_LOOS exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_1d_loos_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = 1.0 / rho[ip];
        let t3 = 1.0 + 0.6166 * t1;
        let t4 = f64::sqrt(t3);
        let t5 = t4 - 1.0;
        let t6 = t5 * t5;
        let t7 = rho[ip] * rho[ip];
        let t8 = t6 * t7;
        let t9 = M_SQRT2;
        let t10 = f64::sqrt(M_PI);
        let t12 = f64::ln(t9 * t10);
        let t14 = -0.3083 * t12 - 0.231225;
        let t15 = t5 * rho[ip];
        let t17 = 1.0 - 3.243593902043464 * t15;
        let t18 = t17 * t17;
        let t22 = -1.2332 * t12 - 0.8632856383593266;
        let t23 = t22 * t5;
        let t29 = t6 * t5;
        let t30 = t7 * rho[ip];
        let t33 = t14 * t18 * t17 + 3.243593902043464 * t23 * rho[ip] * t18 - 1.1985261315879494 * t8 * t17 + 0.2436562958345998 * t29 * t30;
        let t34 = t8 * t33;
        let tzk0 = 10.520901401373546 * t34;
        zk[ip] += tzk0;
    }
}
