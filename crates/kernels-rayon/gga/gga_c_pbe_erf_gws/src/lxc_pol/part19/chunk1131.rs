//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1131/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1131(t13792: f64, t14469: f64, t2503: f64, t3952: f64, t14031: f64, t3224: f64, t3113: f64, t4023: f64, t3283: f64, t4043: f64, t14011: f64, t3242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14470 = t13792 * t14469;
    let t14479 = t3952 * t2503;
    let t14481 = t14031 * t3224;
    let t14483 = t3113 * t4023;
    let t14485 = t4043 * t3283;
    let t14487 = t14011 * t3242;
    (t14470, t14479, t14481, t14483, t14485, t14487)
}
