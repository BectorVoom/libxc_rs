//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 619/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk619<F: Float>(t1434: F, t1444: F, t119: F, t837: F, t84: F, t465: F, t1: F, t1422: F, t467: F, t1425: F, t409: F, t414: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4810 = t1434 * t1444;
    let t4811 = F::cast_from(0.73246220147012639764e-3_f64) * t4810;
    let t4813 = t119 * t837 * t84;
    let t4814 = t465 * t4813;
    let t4815 = F::cast_from(0.56969282336565386482e-3_f64) * t4814;
    let t4816 = t1422 * t1;
    let t4817 = t4816 * t467;
    let t4818 = F::cast_from(0.54934665110259479823e-3_f64) * t4817;
    let t4819 = t409 * t1425;
    let t4820 = F::new(24.0) * t4819;
    let t4821 = t414 * t1425;
    (t4810, t4811, t4813, t4814, t4815, t4816, t4817, t4818, t4819, t4820, t4821)
}
