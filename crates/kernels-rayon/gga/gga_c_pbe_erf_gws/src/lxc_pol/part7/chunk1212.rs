//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1212/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1212(t21419: f64, t2168: f64, t2170: f64, t2171: f64, t2118: f64, t8986: f64, t822: f64, t20515: f64, t3065: f64, t858: f64, t6201: f64, t916: f64) -> (f64, f64, f64) {
    let t21528 = t2168 * t2170 * t21419 * t2171 / 12.0_f64;
    let t21529 = t2118 * t8986;
    let t21530 = t822 * t21529;
    let t21532 = t3065 * t858 * t20515;
    let t21534 = t21530 * t21532 / 8.0_f64;
    let t21535 = t6201 * t916;
    (t21528, t21534, t21535)
}
