//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1201/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1201<F: Float>(t1696: F, t1814: F, t1823: F, t5348: F, t732: F, t5352: F, t5286: F, t21478: F, t234: F, t720: F, t748: F, t1861: F, t5902: F, t1860: F, t5272: F, t1743: F, t1818: F, t1822: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22126 = t1696 * t1814;
    let t22128 = t1696 * t1823;
    let t22130 = t732 * t5348;
    let t22132 = t732 * t5352;
    let t22134 = t732 * t5286;
    let t22139 = 0.17315859105681463759e2 * t234 * t748 * t720 * t21478;
    let t22140 = t5902 * t1861;
    let t22141 = t1860 * t22140;
    let t22143 = t732 * t5272;
    let t22148 = 0.61524113149298439946e4 * t234 * t1818 * t1743 * t1822;
    (t22126, t22128, t22130, t22132, t22134, t22139, t22140, t22141, t22143, t22148)
}
