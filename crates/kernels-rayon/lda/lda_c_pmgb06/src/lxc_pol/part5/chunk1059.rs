//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1059/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1059(t16104: f64, t12038: f64, t19679: f64, t19680: f64, t19681: f64, t19682: f64, t19683: f64, t19685: f64, t19687: f64, t19689: f64, t19693: f64, t16106: f64) -> (f64, f64, f64) {
    let t19694 = 4.0_f64 / 135.0_f64 * t16104;
    let t19695 = t19679 + t19680 - t19681 + t19682 + t19683 + t19685 - t19687 - t19689 - t19693 - t12038 - t19694;
    let t19696 = 4.0_f64 / 135.0_f64 * t16106;
    (t19694, t19695, t19696)
}
