//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 864/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk864(t4991: f64, t597: f64, t1828: f64, t587: f64, t16595: f64, t16597: f64, t16599: f64, t16601: f64, t16603: f64, t16605: f64, t16609: f64, t16611: f64, t16616: f64, t16620: f64) -> (f64, f64) {
    let t16621 = t4991 * t597;
    let t16623 = t587 * t16621 * t1828;
    let t16624 = 32.0_f64 / 135.0_f64 * t16623;
    let t16625 = t16595 + t16597 + t16599 - t16601 - t16603 + t16605 + t16609 - t16611 + t16616 + t16620 + t16624;
    (t16624, t16625)
}
