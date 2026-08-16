//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 238/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk238(t116: f64, t171: f64, t799: f64, t798: f64, t319: f64, param_gamma: f64) -> (f64, f64, f64) {
    let t801 = t799 * t171 * t116;
    let t802 = t798 * t801;
    let t803 = 0.41076328840066666668e0_f64 * t802;
    let t804 = param_gamma * t319;
    (t801, t803, t804)
}
