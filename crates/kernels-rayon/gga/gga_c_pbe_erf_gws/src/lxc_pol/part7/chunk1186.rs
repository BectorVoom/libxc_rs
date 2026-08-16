//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1186/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1186(t2142: f64, t6488: f64, t20802: f64, t875: f64, t2168: f64, t4386: f64, t6084: f64, t817: f64, t2100: f64, t2106: f64, t6095: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21063 = t6488 * t2142;
    let t21064 = 7.0_f64 / 36.0_f64 * t21063;
    let t21065 = t875 * t20802;
    let t21068 = t2168 * t4386 * t21065 / 4.0_f64;
    let t21074 = t6084 * t817;
    let t21077 = t2100 * t2106;
    let t21082 = t814 * t6095;
    (t21064, t21065, t21068, t21074, t21077, t21082)
}
