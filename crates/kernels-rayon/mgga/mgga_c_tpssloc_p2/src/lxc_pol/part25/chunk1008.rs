//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1008/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1008(t22674: f64, t6891: f64, t22892: f64, t1988: f64, t22716: f64, t22724: f64, t6898: f64, t6902: f64, t794: f64, t6897: f64, t22666: f64, t6888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22920 = t22674 * t6891;
    let t22921 = t22892 * t22920;
    let t22923 = t22716 * t1988;
    let t22925 = t22724 * t6898;
    let t22927 = t794 * t6902;
    let t22928 = t6897 * t22927;
    let t22930 = t22666 * t6891;
    let t22931 = t6888 * t22930;
    (t22920, t22921, t22923, t22925, t22927, t22928, t22930, t22931)
}
