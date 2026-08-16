//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1170/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1170(t2133: f64, t6106: f64, t2138: f64, t2263: f64, t339: f64, t824: f64, t822: f64, t20296: f64, t6241: f64, t2121: f64, t337: f64, t6180: f64, t6217: f64) -> (f64, f64, f64) {
    let t20873 = t6106 * t2133;
    let t20875 = t20873 * t2138 / 24.0_f64;
    let t20876 = t339 * t2263;
    let t20877 = t824 * t20876;
    let t20878 = t822 * t20877;
    let t20879 = t20296 * t6241;
    let t20881 = t2121 * t337 * t20879;
    let t20883 = t20878 * t20881 / 4.0_f64;
    let t20885 = t6217 * t6180 / 16.0_f64;
    (t20875, t20883, t20885)
}
