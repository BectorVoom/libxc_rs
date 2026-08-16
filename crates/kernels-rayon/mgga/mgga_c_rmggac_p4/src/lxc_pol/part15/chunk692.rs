//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 692/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk692(t36: f64, t9908: f64, t854: f64, t9876: f64, t851: f64, t9872: f64, t793: f64, t797: f64, t3810: f64, t9888: f64, t3814: f64, t3839: f64, t9884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9909 = t9908 * t36;
    let t9911 = t854 * t9876;
    let t9913 = t851 * t9872;
    let t9915 = t793 * t9872;
    let t9917 = t797 * t9876;
    let t9919 = t3810 * t9888;
    let t9921 = t3814 * t9888;
    let t9923 = t3839 * t9884;
    (t9909, t9911, t9913, t9915, t9917, t9919, t9921, t9923)
}
