//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 962/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk962(t3392: f64, t3395: f64, t6733: f64, t6738: f64, t6740: f64, t6743: f64, t6746: f64, t6750: f64, t6754: f64, t6758: f64, t6763: f64, t6768: f64, t6772: f64, t6777: f64, t6779: f64) -> f64 {
    let t7224 = 8.0_f64 / 3.0_f64 * t3392 + t3395 - t6733 - t6738 - t6740 - t6743 - t6746 - t6750 + t6754 - t6758 - t6763 + t6768 - t6772 - t6777 - t6779;
    t7224
}
