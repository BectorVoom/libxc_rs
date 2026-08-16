//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 814/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk814(t40: f64, t6930: f64, t4: f64, t959: f64, t1448: f64, t2551: f64, t735: f64, t1069: f64, t1617: f64, t2729: f64, t586: f64, t213: f64, t331: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6931 = t40 * t6930;
    let t6932 = 2.0_f64 * t6931;
    let t6967 = t959 * t4;
    let t6968 = t6967 * t1448;
    let t6971 = 4.0_f64 / 45.0_f64 * t2551 * t735;
    let t6998 = t1069 * t1617;
    let t7011 = t2729 * t586;
    let t7018 = t331 * t213;
    (t6932, t6968, t6971, t6998, t7011, t7018)
}
