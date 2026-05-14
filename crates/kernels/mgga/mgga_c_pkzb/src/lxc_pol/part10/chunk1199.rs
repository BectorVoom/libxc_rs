//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1199/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1199<F: Float>(t2887: F, t68: F, t7597: F, t7586: F, t7589: F, t7601: F, t2003: F, t2888: F, t2916: F, t300: F, t779: F, t7707: F, t7710: F, t178: F, t17933: F, t17930: F) -> (F, F, F, F, F, F, F, F) {
    let t21365 = t2887 * t68 * t7597;
    let t21376 = t7586 * t7589;
    let t21387 = t2887 * t68 * t7601;
    let t21395 = t2888 * t2003;
    let t21417 = t300 * t779 * t2916;
    let t21452 = t7707 * t7710;
    let t21454 = t17933 * t178;
    let t21455 = t17930 * t21454;
    (t21365, t21376, t21387, t21395, t21417, t21452, t21454, t21455)
}
