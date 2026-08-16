//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 742/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk742(t4494: f64, t6716: f64, t4501: f64, t743: f64, t833: f64, t3976: f64, t549: f64, t4507: f64, t558: f64, t593: f64, t352: f64, t4515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6717 = t4494 * t6716;
    let t6720 = t4501 * t6716;
    let t6723 = t743 * t833;
    let t6725 = t3976 * t6723 * t549;
    let t6728 = t4507 * t558;
    let t6730 = t6728 * t6723 * t593;
    let t6733 = t6723 * t352;
    let t6734 = t4515 * t6733;
    (t6717, t6720, t6723, t6725, t6728, t6730, t6733, t6734)
}
