//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1089/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1089(t2059: f64, t2060: f64, t279: f64, t6045: f64, t19: f64, t6067: f64, t796: f64, t801: f64, t116: f64, t366: f64, t798: f64, t799: f64) -> (f64, f64, f64) {
    let t19517 = 0.16521134411652656606e2_f64 * t2059 * t2060 * t6045 * t279;
    let t19520 = t6067 * t796 * t19 * t801;
    let t19521 = 0.16430531536026666667e1_f64 * t19520;
    let t19525 = 0.6693920255418271605e1_f64 * t798 * t799 * t366 * t116;
    (t19517, t19521, t19525)
}
