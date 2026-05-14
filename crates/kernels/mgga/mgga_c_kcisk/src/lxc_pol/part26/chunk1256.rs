//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1256/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1256<F: Float>(t113: F, t15217: F, t31894: F, t31898: F, t2927: F, t111242: F, t31895: F, t111228: F, t119: F, t15418: F, t2679: F, t856: F, t31920: F, t3042: F, t912: F, t111237: F, t111259: F) -> (F, F, F, F, F, F, F, F) {
    let t111282 = t15217 * t113 * t31894 * t31898;
    let t111286 = t2927 * t113 * t31894 * t31898;
    let t111288 = t31895 * t111242;
    let t111290 = t31895 * t111228;
    let t111294 = t856 * t15418 * t119 * t2679;
    let t111297 = t2927 * t31920 * t2679;
    let t111301 = t856 * t912 * t3042 * t2679;
    let t111304 = t111237 * t111259;
    (t111282, t111286, t111288, t111290, t111294, t111297, t111301, t111304)
}
