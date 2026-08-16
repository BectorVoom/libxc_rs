//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 745/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk745(t4623: f64, t4800: f64, t470: f64, t1215: f64, t1314: f64, t457: f64, t1434: f64, t1444: f64, t119: f64, t837: f64, t84: f64, t465: f64) -> (f64, f64, f64, f64) {
    let t4801 = t4800 * t4623;
    let t4802 = t470 * t4801;
    let t4803 = 0.51947267698127589897e2_f64 * t4802;
    let t4805 = t1215 * t1314 * t457;
    let t4806 = t470 * t4805;
    let t4807 = 0.35089340384731224426e1_f64 * t4806;
    let t4810 = t1434 * t1444;
    let t4813 = t119 * t837 * t84;
    let t4814 = t465 * t4813;
    (t4803, t4807, t4810, t4814)
}
