//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 590/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk590<F: Float>(t470: F, t4788: F, t1327: F, t414: F, t1319: F, t455: F, t4623: F, t1215: F, t1314: F, t457: F, t119: F, t837: F, t84: F, t465: F, t1333: F, t461: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4789 = t470 * t4788;
    let t4790 = 0.58482233974552040708e0 * t4789;
    let t4798 = t414 * t1327;
    let t4799 = 12.0 * t4798;
    let t4800 = t1319 * t455;
    let t4801 = t4800 * t4623;
    let t4802 = t470 * t4801;
    let t4803 = 0.51947267698127589897e2 * t4802;
    let t4805 = t1215 * t1314 * t457;
    let t4806 = t470 * t4805;
    let t4807 = 0.35089340384731224426e1 * t4806;
    let t4813 = t119 * t837 * t84;
    let t4814 = t465 * t4813;
    let t4815 = 0.56969282336565386482e-3 * t4814;
    let t4825 = t1333 * t461;
    (t4789, t4790, t4798, t4799, t4800, t4801, t4802, t4803, t4805, t4806, t4807, t4813, t4814, t4815, t4825)
}
