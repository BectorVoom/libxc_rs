//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 831/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk831<F: Float>(t6715: F, t683: F, t688: F, t2025: F, t2045: F, t2049: F, t2037: F, t549: F) -> (F, F, F, F) {
    let t6717 = t683 * t6715 * t688;
    let t6720 = t683 * t2025 * t2045;
    let t6723 = t683 * t2025 * t2049;
    let t6725 = t549 * t2037;
    (t6717, t6720, t6723, t6725)
}
