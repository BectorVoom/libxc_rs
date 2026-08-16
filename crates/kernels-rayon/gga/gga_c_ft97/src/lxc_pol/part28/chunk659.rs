//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 659/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk659(t26183: f64, t26228: f64, t26275: f64, t26315: f64, t26363: f64, t26407: f64, t26457: f64, t26490: f64, t1058: f64, t5843: f64, t28: f64, t609: f64, t6718: f64) -> (f64, f64, f64) {
    let t26493 = t26183 + t26228 + t26275 + t26315 + t26363 + t26407 + t26457 + t26490;
    let t26514 = t5843 * t1058;
    let t26515 = t28 * t26514;
    let t26520 = t6718 * t609;
    (t26493, t26515, t26520)
}
