//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1248/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1248(t352: f64, t830: f64, t4002: f64, t8746: f64, t1178: f64, t8713: f64, t2299: f64, t371: f64, t3970: f64, t14425: f64, t51563: f64, t4138: f64, t50948: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53841 = t830 * t352;
    let t53852 = t8746 * t4002;
    let t53860 = t1178 * t8713;
    let t53865 = t3970 * t2299 * t371;
    let t53873 = t51563 * t14425;
    let t53874 = 7.0_f64 / 1152.0_f64 * t53873;
    let t53886 = t50948 * t4138;
    (t53841, t53852, t53860, t53865, t53874, t53886)
}
