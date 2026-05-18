//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 745/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk745<F: Float>(t4623: F, t4800: F, t470: F, t1215: F, t1314: F, t457: F, t1434: F, t1444: F, t119: F, t837: F, t84: F, t465: F) -> (F, F, F, F) {
    let t4801 = t4800 * t4623;
    let t4802 = t470 * t4801;
    let t4803 = F::new(0.51947267698127589897e2) * t4802;
    let t4805 = t1215 * t1314 * t457;
    let t4806 = t470 * t4805;
    let t4807 = F::new(0.35089340384731224426e1) * t4806;
    let t4810 = t1434 * t1444;
    let t4813 = t119 * t837 * t84;
    let t4814 = t465 * t4813;
    (t4803, t4807, t4810, t4814)
}
