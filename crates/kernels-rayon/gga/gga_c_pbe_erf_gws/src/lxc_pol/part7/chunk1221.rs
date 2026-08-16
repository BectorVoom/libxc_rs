//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1221/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1221(t21652: f64, t20851: f64, t21614: f64, t21616: f64, t21627: f64, t21632: f64, t21635: f64, t21640: f64, t21647: f64, t21651: f64, t2337: f64, t3235: f64, t6110: f64, t6282: f64, t902: f64, t905: f64, t9425: f64) -> (f64, f64) {
    let t21653 = 7.0_f64 / 12.0_f64 * t21652;
    let t21658 = t21614 - t21616 + t902 * t905 * t2337 * t6110 / 512.0_f64 + t21627 + t21632 - t21635 + t21640 + t21647 - t21651 - t21653 - 3.0_f64 / 64.0_f64 * t9425 * t3235 * t6282 * t20851;
    (t21653, t21658)
}
