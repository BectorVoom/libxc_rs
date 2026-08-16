//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1159/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1159(t17053: f64, t2655: f64, t164: f64, t20113: f64, t1730: f64, t20199: f64, t2648: f64, t6870: f64, t6892: f64, t1769: f64, t7001: f64, t16324: f64, t177: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20242 = t17053 * t2655;
    let t20252 = t20113 * t164;
    let t20261 = t1730 * t20199 * t2648;
    let t20262 = 0.17006693853500995666e-1_f64 * t20261;
    let t20263 = t6892 * t6870;
    let t20265 = t1769 * t7001;
    let t20267 = t16324 * t177;
    (t20242, t20252, t20262, t20263, t20265, t20267)
}
