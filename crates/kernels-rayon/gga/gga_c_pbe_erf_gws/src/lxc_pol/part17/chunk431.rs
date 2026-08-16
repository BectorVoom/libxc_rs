//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 431/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk431(t617: f64, t649: f64, t661: f64, t1621: f64, t1620: f64, t586: f64, t632: f64) -> (f64, f64, f64, f64) {
    let t1622 = t649 * t617;
    let t1623 = t1622 * t661;
    let t1624 = t1621 * t1623;
    let t1626 = 8.0_f64 / 15.0_f64 * t1620 * t1624;
    let t1627 = t632 * t586;
    (t1623, t1624, t1626, t1627)
}
