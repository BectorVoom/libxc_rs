//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1110/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1110<F: Float>(t1314: F, t3281: F, t5728: F, t8232: F, t92185: F, t93452: F, t1882: F, t23307: F, t23189: F, t23288: F, t5745: F, t23277: F, t23185: F, t23164: F, t23089: F, t5495: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93676 = 28.0 / 81.0 * t3281 * t1314;
    let t93677 = t8232 * t5728;
    let t93728 = 14.0 / 27.0 * t92185;
    let t93776 = 28.0 / 27.0 * t93452;
    let t93815 = t1882 * t23307;
    let t93817 = t1882 * t23189;
    let t93819 = t1882 * t23288;
    let t93828 = t8232 * t5745;
    let t93830 = t1882 * t23277;
    let t93841 = t1882 * t23185;
    let t93843 = t1882 * t23164;
    let t93861 = t5495 * t23089;
    (t93676, t93677, t93728, t93776, t93815, t93817, t93819, t93828, t93830, t93841, t93843, t93861)
}
