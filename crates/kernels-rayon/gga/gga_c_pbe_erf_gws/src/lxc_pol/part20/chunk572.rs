//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 572/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk572(t1109: f64, t817: f64, t1076: f64, t745: f64, t2102: f64, t2107: f64, t2848: f64, t3028: f64, t323: f64, t818: f64) -> (f64, f64, f64) {
    let t3030 = t1109 * t817;
    let t3033 = t1076 * t745;
    let t3037 = -t1076 * t2102 + 2.0_f64 * t2107 * t3033 - t2848 * t818 + t3028 * t323 - t3030 * t745;
    (t3030, t3033, t3037)
}
