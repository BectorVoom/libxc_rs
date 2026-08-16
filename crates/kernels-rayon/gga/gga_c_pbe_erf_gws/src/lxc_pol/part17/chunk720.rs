//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 720/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk720(t376: f64, t814: f64, t810: f64, t353: f64, t4386: f64, t2082: f64, t322: f64, t816: f64, t2352: f64, t2376: f64, t829: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4387 = t376 * t814;
    let t4388 = t4387 * t810;
    let t4389 = t353 * t4388;
    let t4390 = t4386 * t4389;
    let t4394 = 1.0_f64 / t2082 / t322;
    let t4395 = t4394 * t816;
    let t4400 = t2376 * t2352;
    let t4402 = t829 * t830 * t4400;
    (t4387, t4390, t4394, t4395, t4400, t4402)
}
