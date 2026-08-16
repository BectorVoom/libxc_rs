//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 887/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk887(t16704: f64, t1662: f64, t1763: f64, t5292: f64, t56: f64, t5175: f64, t590: f64, t1630: f64, t1791: f64, t5109: f64, t642: f64, t218: f64, t5108: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16705 = 0.19591358024691358025e-1_f64 * t16704;
    let t16712 = 1.0_f64 / t1662 / t1763;
    let t16738 = t56 * t5292;
    let t16739 = t1662 * t1662;
    let t16740 = 1.0_f64 / t16739;
    let t16782 = t590 * t5175;
    let t16797 = t1630 * t1791;
    let t16801 = t642 * t5109;
    let t16823 = 1.0_f64 / t5108 / t218;
    (t16705, t16712, t16738, t16740, t16782, t16797, t16801, t16823)
}
