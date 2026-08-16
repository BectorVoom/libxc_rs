//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 803/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk803(t3138: f64, t6652: f64, t2253: f64, t2277: f64, t6592: f64, t6597: f64, t6600: f64, t6604: f64, t6607: f64, t6614: f64, t6618: f64, t6623: f64, t6625: f64, t6628: f64, t6633: f64, t6637: f64, t6640: f64, t6650: f64) -> (f64, f64) {
    let t6654 = t3138 * t6652 / 16.0_f64;
    let t6655 = -t6592 - t6597 - t2277 * t6600 / 768.0_f64 - t6604 + t6607 + t6614 + t6618 + t6623 - t6625 - 7.0_f64 / 96.0_f64 * t6628 - t2253 * t6633 / 128.0_f64 + t6637 * t6640 / 256.0_f64 + t6650 + t6654;
    (t6654, t6655)
}
