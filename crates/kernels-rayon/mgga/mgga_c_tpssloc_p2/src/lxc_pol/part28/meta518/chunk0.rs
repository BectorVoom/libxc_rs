//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1766/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1766(t22751: f64, t22930: f64, t22917: f64, t22723: f64, t22891: f64, t22920: f64, t117: f64, t5247: f64, t6559: f64, t22674: f64, t22686: f64, t22663: f64, t6883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80665 = t22751 * t22930;
    let t80667 = t22751 * t22917;
    let t80670 = t22723 * t22891;
    let t80671 = t80670 * t22920;
    let t80681 = t6559 * t5247 * t117;
    let t80683 = t80681 * t22674 * t22686;
    let t80689 = t6883 * t22663;
    (t80665, t80667, t80670, t80671, t80681, t80683, t80689)
}
