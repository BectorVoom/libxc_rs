//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 873/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk873<F: Float>(t10043: F, t5542: F, t674: F, t2004: F, t2007: F, t1987: F, t26144: F, t6394: F, t645: F, t26157: F, t6397: F, t1737: F, t352: F, t2060: F, t4044: F, t36978: F, t6382: F, t656: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t46832 = t10043 * t5542;
    let t46833 = t46832 * t674;
    let t46834 = t46833 * t2004;
    let t46836 = t46833 * t2007;
    let t46838 = t46833 * t1987;
    let t46841 = t26144 * t645 * t6394;
    let t46844 = t26157 * t645 * t6397;
    let t46846 = t1737 * t352;
    let t46848 = t4044 * t2060 * t46846;
    let t46853 = t36978 * t656 * t6382;
    (t46832, t46833, t46834, t46836, t46838, t46841, t46844, t46846, t46848, t46853)
}
