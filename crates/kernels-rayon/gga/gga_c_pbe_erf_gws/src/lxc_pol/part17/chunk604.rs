//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 604/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk604(t2784: f64, t598: f64, t186: f64, t185: f64, t1004: f64, t172: f64, t184: f64, t564: f64, t1006: f64, t612: f64, t1883: f64, t582: f64, t996: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2785 = t598 * t2784;
    let t2786 = t186 * t2785;
    let t2788 = 2.0_f64 / 15.0_f64 * t185 * t2786;
    let t2789 = t172 * t1004;
    let t2790 = t2789 * t184;
    let t2792 = 4.0_f64 / 15.0_f64 * t2790 * t564;
    let t2794 = 2.0_f64 / 15.0_f64 * t1006 * t612;
    let t2795 = 8.0_f64 / 45.0_f64 * t1883;
    let t2796 = t582 * t996;
    (t2785, t2786, t2788, t2789, t2790, t2792, t2794, t2795, t2796)
}
