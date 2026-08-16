//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 673/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk673(t1061: f64, t1923: f64, t256: f64, t1918: f64, t2654: f64, t1639: f64, t649: f64, t1642: f64, t1: f64, t837: f64, t1033: f64, t1778: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7733 = t1061 * t1923;
    let t7734 = t7733 * t256;
    let t7736 = t2654 * t1918;
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7776 = t1 * t837;
    let t7811 = t1033 * t1778;
    (t7733, t7734, t7736, t7758, t7759, t7776, t7811)
}
