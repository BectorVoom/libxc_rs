//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 515/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk515(t2641: f64, t769: f64, t1388: f64, t2586: f64, t893: f64, t300: f64, t301: f64, t140: f64, t305: f64, t309: f64) -> (f64, f64, f64, f64) {
    let t3821 = t2641 * t769;
    let t3829 = t2586 * t1388;
    let t3830 = t893 * t3829;
    let t3832 = t300 * t301;
    let t3834 = t305 * t309 * t140;
    let t3835 = t3832 * t3834;
    (t3821, t3829, t3830, t3835)
}
