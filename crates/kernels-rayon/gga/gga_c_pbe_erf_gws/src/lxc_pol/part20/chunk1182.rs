//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1182/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1182(t3788: f64, t4023: f64, t14015: f64, t3754: f64, t3749: f64, t4039: f64, t3783: f64, t14570: f64, t3123: f64, t14007: f64, t3759: f64, t14035: f64, t3837: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15220 = t3788 * t4023;
    let t15222 = t14015 * t3754;
    let t15224 = t4039 * t3749;
    let t15226 = t3783 * t4023;
    let t15228 = t3123 * t14570;
    let t15230 = t14007 * t3759;
    let t15232 = t14035 * t3837;
    (t15220, t15222, t15224, t15226, t15228, t15230, t15232)
}
