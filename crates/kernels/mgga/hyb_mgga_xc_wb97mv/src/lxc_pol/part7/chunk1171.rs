//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1171/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1171<F: Float>(t8143: F, t8155: F, t8160: F, t2987: F, t8135: F, t13: F, t21871: F, t2986: F, t240: F, t6401: F, t92: F, t8140: F, t2991: F, t638: F, t8154: F, t683: F, t8473: F, t8855: F) -> (F, F, F, F, F, F, F) {
    let t25545 = t8160 * t8155 * t8143;
    let t25548 = t2987 * t8155 * t8135;
    let t25551 = t21871 * t13 * t2986;
    let t25556 = t240 * t6401 * t92;
    let t25558 = t8140 * t25556 * t8143;
    let t25565 = t2987 * t8154 * t638 * t2991;
    let t25581 = t683 * t8473 * t8855;
    (t25545, t25548, t25551, t25556, t25558, t25565, t25581)
}
