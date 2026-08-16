//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1118/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1118(t12817: f64, t22917: f64, t5211: f64, t41095: f64, t950: f64, t5218: f64, t7149: f64, t7049: f64, t12599: f64, t24835: f64, t30170: f64, t3406: f64) -> (f64, f64, f64, f64, f64) {
    let t47832 = 32.0_f64 / 9.0_f64 * t5211 * t22917 * t12817;
    let t47833 = t41095 * t950;
    let t47836 = 64.0_f64 / 15.0_f64 * t5218 * t7149 * t47833;
    let t47839 = 32.0_f64 / 9.0_f64 * t5218 * t7049 * t47833;
    let t47841 = 64.0_f64 / 15.0_f64 * t24835 * t12599;
    let t47844 = 32.0_f64 / 15.0_f64 * t5211 * t30170 * t3406;
    (t47832, t47836, t47839, t47841, t47844)
}
