//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 983/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk983(t7957: f64, t7960: f64, t1802: f64, t3443: f64, t610: f64, t1885: f64, t587: f64, t1044: f64, t7019: f64, t7018: f64, t1620: f64, t1037: f64, t7582: f64) -> (f64, f64, f64, f64, f64) {
    let t11108 = 16.0_f64 / 135.0_f64 * t7957;
    let t11109 = 16.0_f64 / 45.0_f64 * t7960;
    let t11110 = t1802 * t3443;
    let t11111 = t11110 * t610;
    let t11112 = t1885 * t11111;
    let t11114 = 4.0_f64 / 15.0_f64 * t587 * t11112;
    let t11115 = t7019 * t1044;
    let t11116 = t7018 * t11115;
    let t11118 = 8.0_f64 / 15.0_f64 * t1620 * t11116;
    let t11120 = 8.0_f64 / 45.0_f64 * t7582 * t1037;
    (t11108, t11109, t11114, t11118, t11120)
}
