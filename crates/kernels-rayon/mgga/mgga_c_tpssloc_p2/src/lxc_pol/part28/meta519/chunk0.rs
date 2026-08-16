//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1767/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1767(t214: f64, t3879: f64, t22675: f64, t22724: f64, t22716: f64, t6903: f64, t22662: f64, t22674: f64, t6897: f64, t22684: f64, t6546: f64, t22687: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80707 = t214 * t3879;
    let t80711 = t22724 * t22675;
    let t80722 = t22716 * t6903;
    let t80725 = t6897 * t22674 * t22662;
    let t80727 = t6546 * t22684;
    let t80728 = t80727 * t22687;
    (t80707, t80711, t80722, t80725, t80727, t80728)
}
