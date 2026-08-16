//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1205/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1205(t22666: f64, t22685: f64, t22686: f64, t117: f64, t5247: f64, t6559: f64, t22674: f64, t1985: f64, t22662: f64, t22663: f64, t6883: f64, t225: f64, t22624: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80678 = t22685 * t22666 * t22686;
    let t80681 = t6559 * t5247 * t117;
    let t80683 = t80681 * t22674 * t22686;
    let t80687 = t1985 * t22666 * t22662;
    let t80689 = t6883 * t22663;
    let t80699 = t22624 * t225;
    (t80678, t80681, t80683, t80687, t80689, t80699)
}
