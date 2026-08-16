//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1115/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1115(t2379: f64, t4453: f64, t2246: f64, t4446: f64, t2242: f64, t2420: f64, t2271: f64, t4422: f64, t822: f64, t833: f64, t4414: f64, t6140: f64) -> (f64, f64, f64, f64, f64) {
    let t19999 = t4453 * t2379;
    let t20007 = t2246 * t4446;
    let t20009 = t2242 * t2420;
    let t20015 = t2271 * t4422;
    let t20017 = t822 * t20015 * t833;
    let t20024 = t4414 * t6140;
    (t19999, t20007, t20009, t20017, t20024)
}
