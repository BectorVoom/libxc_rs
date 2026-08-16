//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 741/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk741(t22724: f64, t6973: f64, t6982: f64, t794: f64, t6897: f64, t6883: f64, t6983: f64, t6914: f64, t6979: f64, t6546: f64, t6887: f64) -> (f64, f64, f64, f64, f64) {
    let t22725 = t22724 * t6973;
    let t22727 = t794 * t6982;
    let t22728 = t6897 * t22727;
    let t22730 = t6883 * t6983;
    let t22731 = 0.38381794893125283518e-1_f64 * t22730;
    let t22745 = t6914 * t6979;
    let t22746 = 0.38381794893125283518e-1_f64 * t22745;
    let t22751 = t6546 * t6887;
    (t22725, t22728, t22731, t22746, t22751)
}
