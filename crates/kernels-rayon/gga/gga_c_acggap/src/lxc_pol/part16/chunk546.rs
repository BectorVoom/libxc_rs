//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 546/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk546(t4045: f64, t1: f64, t1378: f64, t283: f64, t2894: f64, t1390: f64, t229: f64, t276: f64, t40: f64, t1388: f64, t483: f64, t803: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4046 = 8.0_f64 * t4045;
    let t4047 = t1378 * t1;
    let t4048 = t4047 * t283;
    let t4049 = 0.36622894612013090108e-3_f64 * t4048;
    let t4050 = 12.0_f64 * t2894;
    let t4057 = t229 * t1390;
    let t4059 = t1378 * t276;
    let t4060 = t40 * t4059;
    let t4061 = 2.0_f64 * t4060;
    let t4062 = t229 * t1388;
    let t4063 = 8.0_f64 * t4062;
    let t4068 = t483 * t803;
    (t4046, t4049, t4050, t4057, t4061, t4063, t4068)
}
