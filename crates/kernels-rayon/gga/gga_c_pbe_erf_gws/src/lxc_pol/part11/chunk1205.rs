//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1205/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1205(t18527: f64, t18529: f64, t18556: f64, t18562: f64, t18567: f64, t18571: f64, t18574: f64, t48479: f64, t48480: f64, t48481: f64, t48482: f64, t48483: f64, t48484: f64, t48485: f64, t48486: f64, t48488: f64) -> f64 {
    let t48950 = t48479 + t18527 - t18529 - t48480 - t48481 - t48482 - t48483 - t48484 + t48485 - t48486 - t18556 - t18562 + t48488 + t18567 + t18571 - t18574;
    t48950
}
