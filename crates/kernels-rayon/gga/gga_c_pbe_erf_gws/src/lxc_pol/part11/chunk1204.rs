//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1204/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1204(t1168: f64, t13086: f64, t18512: f64, t18518: f64, t19482: f64, t2429: f64, t3703: f64, t3717: f64, t3718: f64, t3929: f64, t3932: f64, t47181: f64, t48441: f64, t48442: f64, t48443: f64, t48444: f64, t48445: f64, t48446: f64, t48474: f64, t48475: f64, t48478: f64, t804: f64) -> f64 {
    let t48948 = 12.0_f64 * t1168 * t13086 * t804 + 36.0_f64 * t2429 * t3703 * t3929 - 18.0_f64 * t3717 * t3932 * t804 + 36.0_f64 * t3718 * t47181 + t18512 + t18518 + t19482 + t48441 + t48442 - t48443 + t48444 + t48445 - t48446 + t48474 - t48475 - t48478;
    t48948
}
