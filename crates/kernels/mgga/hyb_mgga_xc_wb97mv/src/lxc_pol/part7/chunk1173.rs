//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1173/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1173<F: Float>(t1313: F, t136: F, t3003: F, t1234: F, t6494: F, t549: F, t8847: F, t25303: F, t3168: F, t683: F, t685: F, t1237: F, t3167: F, t2037: F, t3178: F, t6715: F) -> (F, F, F, F, F, F, F) {
    let t25603 = t136 * t3003 * t1313;
    let t25614 = t1234 * t6494;
    let t25627 = t136 * t549 * t8847;
    let t25633 = t683 * t25303 * t685 * t3168;
    let t25636 = t683 * t3167 * t1237;
    let t25652 = t3003 * t2037;
    let t25657 = t683 * t6715 * t3178;
    (t25603, t25614, t25627, t25633, t25636, t25652, t25657)
}
