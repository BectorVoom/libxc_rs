//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 684/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk684(t1964: f64, t992: f64, t2030: f64, t987: f64, t475: f64, t2936: f64, t751: f64, t1: f64, t1098: f64, t2057: f64, t2062: f64, t1167: f64, t804: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8490 = t992 * t1964;
    let t8496 = t987 * t2030;
    let t8497 = t475 * t8496;
    let t8503 = t751 * t2936;
    let t8519 = t1098 * t2057 * t1;
    let t8520 = t8519 * t2062;
    let t8555 = t804 * t1167;
    (t8490, t8496, t8497, t8503, t8519, t8520, t8555)
}
