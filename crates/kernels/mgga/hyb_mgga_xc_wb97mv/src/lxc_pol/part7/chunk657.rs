//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 657/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk657<F: Float>(t143: F, t1264: F, t1279: F, t172: F, t187: F, t3243: F, t3244: F, t3284: F, t739: F, t758: F, t139: F, t214: F, t26: F, t1313: F, t549: F, t1312: F, t191: F) -> (F, F, F, F, F, F) {
    let t144 = 0.135e1 <= t143;
    let t3288 = piecewise3(t144, t3243, -8.0 / 3.0 * t1264 * t758 - 8.0 / 3.0 * t739 * t1279 - 8.0 / 3.0 * t172 * t3284 - 8.0 / 3.0 * t3244 * t187);
    let t3289 = t139 * t3288;
    let t3290 = t3289 * t214;
    let t3291 = t26 * t3290;
    let t3296 = t549 * t1313;
    let t3299 = t191 * t1312;
    (t3288, t3289, t3290, t3291, t3296, t3299)
}
