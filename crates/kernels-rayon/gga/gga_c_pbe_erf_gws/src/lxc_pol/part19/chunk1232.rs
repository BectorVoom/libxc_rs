//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1232/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1232(t14602: f64, t51666: f64, t14415: f64, t51563: f64, t14127: f64, t2503: f64, t51530: f64, t13791: f64, t3039: f64, t1144: f64, t4387: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53597 = t51666 * t14602;
    let t53625 = t51563 * t14415;
    let t53645 = t14127 * t2503;
    let t53666 = 119.0_f64 / 1728.0_f64 * t51530;
    let t53688 = t3039 * t13791;
    let t53699 = t859 * t1144 * t4387;
    (t53597, t53625, t53645, t53666, t53688, t53699)
}
