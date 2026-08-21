//! LDA_C_1D_LOOS exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_1d_loos.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI, M_SQRT2};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_1d_loos_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = 1.0 / t1;
        let t4 = 1.0 + 0.6166 * t2;
        let t5 = rmath::sqrt(t4);
        let t6 = t5 - 1.0;
        let t7 = t6 * t6;
        let t8 = t1 * t1;
        let t9 = t7 * t8;
        let t10 = M_SQRT2;
        let t11 = rmath::sqrt(M_PI);
        let t13 = rmath::ln(t10 * t11);
        let t15 = -0.3083 * t13 - 0.231225;
        let t16 = t6 * t1;
        let t18 = 1.0 - 3.243593902043464 * t16;
        let t19 = t18 * t18;
        let t23 = -1.2332 * t13 - 0.8632856383593266;
        let t24 = t23 * t6;
        let t30 = t7 * t6;
        let t31 = t8 * t1;
        let t34 = t15 * t19 * t18 + 3.243593902043464 * t24 * t1 * t19 - 1.1985261315879494 * t9 * t18 + 0.2436562958345998 * t30 * t31;
        let t35 = t9 * t34;
        let tzk0 = 10.520901401373546 * t35;
        zk[ip] += tzk0;
    }
}
