//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 977/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk977(t34053: f64, t870: f64, t34074: f64, t8392: f64, t34078: f64, t34070: f64, t34204: f64, t7584: f64, t10696: f64, t7672: f64, t7662: f64, t848: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t143592 = t870 * t34053;
    let t143604 = t8392 * t34074;
    let t143606 = t8392 * t34078;
    let t143608 = t8392 * t34070;
    let t143610 = t8392 * t34204;
    let t143612 = t870 * t7584;
    let t143621 = t10696 * t7672;
    let t143653 = t848 * t7662;
    (t143592, t143604, t143606, t143608, t143610, t143612, t143621, t143653)
}
