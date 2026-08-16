//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 804/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk804(t101: f64, t12989: f64, t12436: f64, t12438: f64, t12442: f64, t12446: f64, t12448: f64, t12450: f64, t12454: f64, t12488: f64, t12521: f64, t12524: f64, t12525: f64, t12530: f64, t4872: f64, t4876: f64, t4910: f64, t6998: f64, t7075: f64) -> (f64, f64) {
    let t12990 = t101 * t12989;
    let t13005 = -t4872 + 2.0_f64 / 45.0_f64 * t6998 + t4876 + t12436 - t12438 - t12442 - t12446 + t12448 + t12450 + t12454 + t4910 + t12488 + t12521 + 4.0_f64 / 3.0_f64 * t7075 + t12524 - t12525 + t12530;
    (t12990, t13005)
}
