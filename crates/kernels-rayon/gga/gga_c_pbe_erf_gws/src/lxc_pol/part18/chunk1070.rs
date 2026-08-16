//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1070/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1070(t12024: f64, t906: f64, t11819: f64, t8599: f64, t2168: f64, t11990: f64, t4386: f64, t2127: f64, t3781: f64, t850: f64, t860: f64, t2142: f64, t3788: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12025 = t12024 * t906;
    let t12029 = t8599 * t11819;
    let t12031 = t2168 * t12029 / 8.0_f64;
    let t12032 = t4386 * t11990;
    let t12034 = t2168 * t12032 / 24.0_f64;
    let t12036 = t850 * t3781 * t2127;
    let t12038 = t12036 * t860 / 96.0_f64;
    let t12039 = t3788 * t2142;
    (t12025, t12031, t12034, t12036, t12038, t12039)
}
