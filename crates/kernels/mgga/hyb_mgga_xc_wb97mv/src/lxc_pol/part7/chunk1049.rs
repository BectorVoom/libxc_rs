//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1049/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1049<F: Float>(t143: F, t10673: F, t10760: F, t10762: F, t10839: F, t1264: F, t1279: F, t172: F, t187: F, t3244: F, t3284: F, t4026: F, t4062: F, t739: F, t758: F, t139: F, t214: F) -> (F, F) {
    let t144 = 0.135e1 <= t143;
    let t10843 = piecewise3(t144, t10673 + t10760, -8.0 / 3.0 * t10762 * t187 - 8.0 / 3.0 * t4026 * t758 - 16.0 / 3.0 * t3244 * t1279 - 16.0 / 3.0 * t1264 * t3284 - 8.0 / 3.0 * t739 * t4062 - 8.0 / 3.0 * t172 * t10839);
    let t10844 = t139 * t10843;
    let t10845 = t10844 * t214;
    (t10843, t10845)
}
