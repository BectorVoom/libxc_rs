//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 611/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk611<F: Float>(t470: F, t4801: F, t1215: F, t1314: F, t457: F, t119: F, t837: F, t84: F, t465: F, t1333: F, t461: F, t1319: F, t456: F, t4607: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4802 = t470 * t4801;
    let t4803 = F::cast_from(0.51947267698127589897e2_f64) * t4802;
    let t4805 = t1215 * t1314 * t457;
    let t4806 = t470 * t4805;
    let t4807 = F::cast_from(0.35089340384731224426e1_f64) * t4806;
    let t4813 = t119 * t837 * t84;
    let t4814 = t465 * t4813;
    let t4815 = F::cast_from(0.56969282336565386482e-3_f64) * t4814;
    let t4825 = t1333 * t461;
    let t4826 = F::cast_from(60.0_f64) * t4825;
    let t4835 = t1319 * t4607 * t456;
    (t4802, t4803, t4805, t4806, t4807, t4813, t4814, t4815, t4825, t4826, t4835)
}
