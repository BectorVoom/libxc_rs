//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 852/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk852(t3620: f64, t377: f64, t1289: f64, t1295: f64, t3631: f64, t374: f64, t3630: f64, t67: f64, t73: f64, t2786: f64, t56: f64, t69: f64) -> (f64, f64, f64, f64, f64) {
    let t8396 = t3620 * t377;
    let t8399 = t1289 * t1295;
    let t8404 = t374 * t3631;
    let t8413 = t67 / t3630 / t73;
    let t8428 = 2.9801938271604937_f64 * t69 * t2786 * t56;
    (t8396, t8399, t8404, t8413, t8428)
}
