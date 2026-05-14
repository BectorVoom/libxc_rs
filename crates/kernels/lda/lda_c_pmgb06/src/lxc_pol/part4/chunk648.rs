//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 648/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk648<F: Float>(t1180: F, t242: F, t30: F, t3667: F, t633: F, t1041: F, t409: F, t621: F, t138: F, t634: F, t1018: F, t1036: F, t1040: F, t109: F, t1044: F, t1003: F, t1009: F, t1054: F, t1055: F, t1061: F, t269: F, t282: F, t3719: F, t3834: F, t3842: F, t3851: F, t3859: F, t3862: F, t3867: F, t3871: F, t666: F, t668: F, t991: F, t992: F, t994: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3874 = 0.0034450798614814814 * t30 * t1180 * t242;
    let t3875 = t3667 * t633;
    let t3877 = 6.0 * t1041 * t3875;
    let t3878 = t409 * t621;
    let t3881 = 0.07123333333333333 * t138 * t3878 * t634;
    let t3884 = 0.053425 * t138 * t1018 * t1036;
    let t3885 = t109 * t1040;
    let t3888 = 0.8591797547176487 * t138 * t3885 * t1044;
    let t3889 = 0.03253074390090522 * t138 * t3834 * t1055 + 0.10274 * t138 * t109 * t991 * t994 - t3719 + 3.5089341735807875 * t1061 * t3842 - 6.0 * t992 * t668 * t1003 + 0.0016562821945185185 * t30 * t1180 * t269 + 96.49187699215521 * t1009 * t3851 * t666 + 0.0005696894717424259 * t30 * t1180 * t282 + 51.94757731704439 * t1061 * t3859 - 3.5089341735807875 * t1054 * t3862 + t3867 - t3871 - t3874 - t3877 - t3881 + t3884 + t3888;
    (t3874, t3875, t3877, t3878, t3881, t3884, t3885, t3888, t3889)
}
