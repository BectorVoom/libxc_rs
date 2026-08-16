//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 843/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk843(t13086: f64, t339: f64, t1130: f64, t11706: f64, t13156: f64, t13325: f64, t13328: f64, t2181: f64, t3154: f64, t340: f64, t3848: f64, t3851: f64, t6429: f64, t870: f64, t9056: f64) -> (f64, f64) {
    let t13331 = t339 * t13086;
    let t13334 = -t13156 * t339 * t340 + 9.0_f64 * t1130 * t11706 + 60.0_f64 * t13325 * t6429 - 36.0_f64 * t13328 * t2181 + 3.0_f64 * t13331 * t870 + 9.0_f64 * t3154 * t3851 - 36.0_f64 * t3848 * t9056;
    (t13331, t13334)
}
