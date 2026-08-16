//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1177/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1177(t16572: f64, t16574: f64, t47527: f64, t47528: f64, t47529: f64, t47530: f64, t47535: f64, t47536: f64, t47538: f64, t47545: f64, t47546: f64, t16595: f64, t47547: f64, t47548: f64, t47552: f64, t47554: f64, t47555: f64, t47559: f64, t47560: f64, t47561: f64, t47562: f64, t47565: f64) -> (f64, f64) {
    let t48629 = t47527 - t47528 + t47529 + t47530 + t47535 - t47536 - t47538 + t16572 - t47545 + t16574 + t47546;
    let t48630 = t47547 - t47548 + t16595 + t47552 + t47554 - t47555 - t47559 + t47560 + t47561 + t47562 + t47565;
    (t48629, t48630)
}
