//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 827/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk827(t1: f64, t297: f64, t7835: f64, t313: f64, t2586: f64, t2590: f64, t893: f64, t2597: f64, t6541: f64, t897: f64, t894: f64, t224: f64, t2269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7837 = t7835 * t1 * t297;
    let t7838 = t313 * t7837;
    let t7845 = t2586 * t2590;
    let t7846 = t893 * t7845;
    let t7848 = t2586 * t2597;
    let t7849 = t893 * t7848;
    let t7851 = t897 * t6541;
    let t7852 = t894 * t7851;
    let t7856 = 1.0_f64 / t224 / t2269;
    (t7837, t7838, t7845, t7846, t7848, t7849, t7851, t7852, t7856)
}
