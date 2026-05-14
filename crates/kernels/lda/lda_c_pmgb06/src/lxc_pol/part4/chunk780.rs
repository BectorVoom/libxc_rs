//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 780/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk780<F: Float>(t5770: F, t5772: F, t2233: F, t342: F, t1227: F, t780: F, t348: F, t776: F, t110: F, t2217: F, t360: F, t2186: F, t947: F, t410: F, t365: F, t350: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5774 = 5.87616 * t5770 * t5772;
    let t5775 = t2233 * t342;
    let t5779 = t780 * t1227;
    let t5783 = t348 * t776;
    let t5785 = 1.9486833333333333 * t5783 * t5772;
    let t5787 = t360 * t110 * t2217;
    let t5788 = t2186 * t947;
    let t5789 = 0.6495611111111111 * t5788;
    let t5790 = t410 * t776;
    let t5791 = t360 * t5790;
    let t5793 = t365 * t2233;
    let t5795 = 1.46904 * t5793 * t350;
    (t5774, t5775, t5779, t5783, t5785, t5787, t5788, t5789, t5790, t5791, t5793, t5795)
}
