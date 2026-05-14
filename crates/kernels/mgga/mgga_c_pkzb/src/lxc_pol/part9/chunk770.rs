//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 770/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk770<F: Float>(t197: F, t5718: F, t2021: F, t271: F, t296: F, t294: F, t46: F, t2027: F, t759: F) -> (F, F, F, F, F, F) {
    let t5719 = t5718 * t197;
    let t5722 = 1.0 / t2021 / t296 / t271;
    let t5723 = t294 * t5722;
    let t5724 = t5723 * t46;
    let t5725 = t5719 * t5724;
    let t5726 = t2027 * t759;
    (t5719, t5722, t5723, t5724, t5725, t5726)
}
