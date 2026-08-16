//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1225/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1225(t20514: f64, t20549: f64, t20553: f64, t20557: f64, t20564: f64, t20566: f64, t20569: f64, t20582: f64, t20584: f64, t20588: f64, t20593: f64, t20601: f64, t20606: f64, t20615: f64, t20623: f64, t20631: f64, t20638: f64, t20653: f64, t20658: f64, t20669: f64, t20691: f64, t20700: f64) -> (f64, f64) {
    let t21693 = t20514 + t20549 - t20553 - t20557 - t20564 - t20566 + t20569 - t20582 + t20584 - t20588 - t20593;
    let t21694 = t20601 - t20606 + t20615 - t20623 + t20631 + t20638 - t20653 - t20658 + t20669 - t20691 - t20700;
    (t21693, t21694)
}
