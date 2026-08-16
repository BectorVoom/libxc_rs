//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 567/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk567(t1130: f64, t2181: f64, t3154: f64, t339: f64, t340: f64, t3772: f64, t3848: f64, t3851: f64, t870: f64) -> f64 {
    let t3854 = -t339 * t340 * t3772 + 6.0_f64 * t1130 * t3154 - 12.0_f64 * t2181 * t3848 + 3.0_f64 * t3851 * t870;
    t3854
}
