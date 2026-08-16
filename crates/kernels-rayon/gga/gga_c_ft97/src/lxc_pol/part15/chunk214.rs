//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 214/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk214(t663: f64, t231: f64, t294: f64, t301: f64, t342: f64, t343: f64, t10: f64, t296: f64, t351: f64, t295: f64, t668: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t764 = t663 / 3.0_f64;
    let t784 = t231 * t294;
    let t788 = t301 - t342 * t343 * t784 / 4.0_f64;
    let t790 = t10 * t351 * t296;
    let t791 = t790 / 18.0_f64;
    let t792 = t295 * t668;
    let t797 = t294 * t294;
    let t798 = 1.0_f64 / t797;
    (t764, t784, t788, t790, t791, t792, t797, t798)
}
