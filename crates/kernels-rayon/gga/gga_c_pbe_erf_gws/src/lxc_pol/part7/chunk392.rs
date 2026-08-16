//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 392/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk392(t1630: f64, t644: f64, t639: f64, t1416: f64, t643: f64, t642: f64, t212: f64, t626: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1631 = t1630 * t644;
    let t1632 = t639 * t1631;
    let t1633 = 16.0_f64 / 135.0_f64 * t1632;
    let t1634 = t643 * t1416;
    let t1635 = t642 * t1634;
    let t1637 = 4.0_f64 / 45.0_f64 * t639 * t1635;
    let t1639 = 1.0_f64 / t212 / t626;
    (t1631, t1633, t1634, t1635, t1637, t1639)
}
