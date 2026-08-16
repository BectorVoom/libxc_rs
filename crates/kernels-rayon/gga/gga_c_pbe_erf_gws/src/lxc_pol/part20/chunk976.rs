//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 976/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk976(t3555: f64, t633: f64, t3402: f64, t4934: f64, t1620: f64, t3406: f64, t5137: f64, t639: f64, t3554: f64, t582: f64, t211: f64, t2601: f64, t2722: f64) -> (f64, f64, f64, f64, f64) {
    let t11018 = 2.0_f64 / 15.0_f64 * t633 * t3555;
    let t11019 = t4934 * t3402;
    let t11020 = t1620 * t11019;
    let t11021 = 32.0_f64 / 135.0_f64 * t11020;
    let t11022 = t5137 * t3406;
    let t11023 = t639 * t11022;
    let t11024 = 16.0_f64 / 135.0_f64 * t11023;
    let t11025 = t582 * t3554;
    let t11026 = t211 * t11025;
    let t11027 = 4.0_f64 / 45.0_f64 * t11026;
    let t11028 = t2601 * t2722;
    (t11018, t11021, t11024, t11027, t11028)
}
