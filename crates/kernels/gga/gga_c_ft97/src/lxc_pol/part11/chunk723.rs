//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 723/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk723<F: Float>(t265: F, t729: F, t9692: F, t731: F, t8232: F, t768: F, t1882: F, t2563: F, t2559: F, t724: F, t9587: F, t2594: F, t9578: F) -> (F, F, F, F, F, F, F) {
    let t9819 = t729 * t265 * t9692;
    let t9822 = t8232 * t731;
    let t9824 = t8232 * t768;
    let t9826 = t1882 * t2563;
    let t9828 = t1882 * t2559;
    let t9831 = t724 * t265 * t9587;
    let t9835 = t2594 * t265 * t9578;
    (t9819, t9822, t9824, t9826, t9828, t9831, t9835)
}
