//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 873/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk873(t13456: f64, t13457: f64, t13459: f64, t13465: f64, t13470: f64, t13475: f64, t13478: f64, t13479: f64, t13485: f64, t13486: f64, t13488: f64, t13493: f64, t13498: f64, t13503: f64) -> f64 {
    let t13674 = t13456 - t13457 - t13459 - t13465 + t13470 - t13475 + t13478 + t13479 + t13485 - t13486 - t13488 - t13493 + t13498 + t13503;
    t13674
}
