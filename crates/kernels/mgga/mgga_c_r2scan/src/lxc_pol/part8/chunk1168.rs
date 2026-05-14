//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1168/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1168<F: Float>(t21234: F, t5460: F, t721: F, t1982: F, t5530: F, t61: F, t21248: F, t1673: F, t5465: F, t1266: F, t1680: F, t1684: F, t21: F, t5917: F, t5967: F, t1762: F, t2021: F, t5916: F) -> (F, F, F, F, F, F, F) {
    let t22242 = 0.37402255668271961718e4 * t5460 * t721 * t21234;
    let t22246 = 0.73828935779158127934e5 * t61 * t5530 * t1982 * t21234;
    let t22249 = 0.41016075432865626632e4 * t5460 * t1982 * t21248;
    let t22250 = t1673 * t5465;
    let t22255 = 0.75272354370370370365e-2 * t1680 * t1684 * t21 * t1266;
    let t22260 = t5967 * t5917;
    let t22264 = 0.26024595120724175621e0 * t1762 * t5916 * t2021;
    (t22242, t22246, t22249, t22250, t22255, t22260, t22264)
}
