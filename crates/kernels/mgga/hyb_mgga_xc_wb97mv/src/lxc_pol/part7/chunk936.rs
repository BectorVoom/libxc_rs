//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 936/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk936<F: Float>(t143: F, t1264: F, t1279: F, t172: F, t187: F, t2115: F, t2158: F, t3244: F, t3284: F, t739: F, t758: F, t8721: F, t8759: F, t8761: F, t8841: F, t139: F, t214: F) -> (F, F, F) {
    let t144 = 0.135e1 <= t143;
    let t8845 = piecewise3(t144, t8721 + t8759, -8.0 / 3.0 * t8761 * t187 - 16.0 / 3.0 * t3244 * t758 - 8.0 / 3.0 * t1264 * t2158 - 8.0 / 3.0 * t2115 * t1279 - 16.0 / 3.0 * t739 * t3284 - 8.0 / 3.0 * t172 * t8841);
    let t8846 = t139 * t8845;
    let t8847 = t8846 * t214;
    (t8845, t8846, t8847)
}
