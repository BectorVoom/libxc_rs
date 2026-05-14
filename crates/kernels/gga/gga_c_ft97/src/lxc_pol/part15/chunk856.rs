//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 856/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk856<F: Float>(t1882: F, t21443: F, t21458: F, t21454: F, t1775: F, t21613: F, t21607: F, t21599: F, t21610: F, t21573: F, t21581: F, t2: F, t21399: F, t21597: F, t21595: F, t21592: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t80772 = t1882 * t21443;
    let t80819 = t1882 * t21458;
    let t80821 = t1882 * t21454;
    let t80893 = t1775 * t21613;
    let t80911 = t1775 * t21607;
    let t80913 = t1775 * t21599;
    let t80915 = t1775 * t21610;
    let t80942 = t1775 * t21573;
    let t80961 = t1775 * t21581;
    let t80963 = t2 * t21399;
    let t81006 = t1775 * t21597;
    let t81008 = t1775 * t21595;
    let t81010 = t1775 * t21592;
    (t80772, t80819, t80821, t80893, t80911, t80913, t80915, t80942, t80961, t80963, t81006, t81008, t81010)
}
