//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 937/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk937(t336: f64, t9239: f64, t2263: f64, t339: f64, t824: f64, t2262: f64, t359: f64, t362: f64, t366: f64, t899: f64, t2157: f64, t2264: f64, t2331: f64) -> (f64, f64, f64, f64, f64) {
    let t20842 = t9239 * t336;
    let t20876 = t339 * t2263;
    let t20877 = t824 * t20876;
    let t20930 = 1.0_f64 / t2262 / t359 * t362;
    let t20932 = t899 * t20930 * t366;
    let t20933 = t2157 * t2157;
    let t20940 = t899 * t2264 * t2331;
    (t20842, t20877, t20932, t20933, t20940)
}
