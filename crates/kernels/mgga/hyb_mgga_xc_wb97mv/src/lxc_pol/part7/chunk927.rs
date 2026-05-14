//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 927/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk927<F: Float>(t2044: F, t3: F, t2025: F, t3178: F, t683: F, t3163: F, t685: F, t8473: F, t3168: F, t1232: F, t6701: F, t3141: F, t8195: F, t1313: F, t2003: F, t136: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8595 = t2044 * t3;
    let t8601 = t683 * t2025 * t3178 / 96.0;
    let t8604 = t683 * t2025 * t3163 / 96.0;
    let t8605 = t8473 * t685;
    let t8607 = t683 * t8605 * t3168;
    let t8609 = t6701 * t1232;
    let t8617 = t8195 * t3141;
    let t8619 = t2003 * t1313;
    let t8620 = t136 * t8619;
    (t8595, t8601, t8604, t8605, t8607, t8609, t8617, t8619, t8620)
}
