//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 672/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk672<F: Float>(t28748: F, t6317: F, t1091: F, t25165: F, t2665: F, t1485: F, t3051: F, t3746: F, t6318: F, t684: F, t7036: F, t24976: F) -> (F, F, F, F, F, F) {
    let t28749 = t6317 * t28748;
    let t28752 = t2665 * t25165 * t1091;
    let t28753 = t6317 * t28752;
    let t28755 = t1485 * t3051;
    let t28757 = t2665 * t6318 * t3746;
    let t28758 = t28755 * t28757;
    let t28760 = t7036 * t684;
    let t28761 = t24976 * t28760;
    (t28749, t28753, t28755, t28758, t28760, t28761)
}
