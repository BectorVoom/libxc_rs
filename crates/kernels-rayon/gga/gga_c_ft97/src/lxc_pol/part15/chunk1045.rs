//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1045/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1045(t1526: f64, t1527: f64, t19970: f64, t20022: f64, t20098: f64, t20141: f64, t20145: f64, t20163: f64, t342: f64, t343: f64, t61180: f64, t61184: f64, t72: f64, t75935: f64, t75944: f64, t75947: f64, t7712: f64) -> f64 {
    let t86508 = t19970 - t75944 / 12.0_f64 + t75947 / 6.0_f64 + t20163 - t342 * t343 * t72 * t20098 / 4.0_f64 - t75935 / 4.0_f64 + t61180 / 6.0_f64 + t61184 / 18.0_f64 - t1526 * t1527 * t20145 / 4.0_f64 - t1526 * t1527 * t20141 / 4.0_f64 - t1526 * t1527 * t7712 * t20022 / 2.0_f64;
    t86508
}
