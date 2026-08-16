//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1906/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1906(t22635: f64, t26332: f64, t3734: f64, t90591: f64, t22916: f64, t26193: f64, t6888: f64, t22633: f64, t26354: f64, t90506: f64, t26211: f64, t6883: f64) -> (f64, f64, f64, f64) {
    let t90594 = t90591 * t22635 * t26332 * t3734;
    let t90598 = t6888 * t26193 * t22916;
    let t90602 = t22633 * t22635 * t26354 * t90506;
    let t90604 = t6883 * t26211;
    (t90594, t90598, t90602, t90604)
}
