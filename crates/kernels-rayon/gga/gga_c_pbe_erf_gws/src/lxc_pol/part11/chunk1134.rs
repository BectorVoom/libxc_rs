//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1134/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1134(t12617: f64, t2612: f64, t12620: f64, t12623: f64, t12627: f64, t7527: f64, t12818: f64, t48067: f64, t48069: f64, t48071: f64, t48076: f64, t48078: f64, t48080: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48082 = 16.0_f64 / 15.0_f64 * t2612 * t12617;
    let t48084 = 32.0_f64 / 15.0_f64 * t2612 * t12620;
    let t48086 = 16.0_f64 / 9.0_f64 * t2612 * t12623;
    let t48088 = 32.0_f64 / 9.0_f64 * t7527 * t12627;
    let t48090 = 16.0_f64 / 9.0_f64 * t2612 * t12818;
    let t48091 = t48067 + t48069 + t48071 + t48076 + t48078 + t48080 - t48082 - t48084 + t48086 + t48088 - t48090;
    (t48082, t48084, t48086, t48088, t48090, t48091)
}
