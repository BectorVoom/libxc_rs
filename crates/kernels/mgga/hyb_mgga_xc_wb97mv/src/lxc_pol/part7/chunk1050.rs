//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1050/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1050<F: Float>(t10845: F, t26: F, t10589: F, t10594: F, t10599: F, t10604: F, t10607: F, t10609: F, t10613: F, t10617: F, t10622: F, t10626: F, t136: F, t2035: F, t2038: F, t3167: F, t4073: F, t6492: F, t676: F, t683: F, t686: F, t8561: F) -> (F, F) {
    let t10846 = t26 * t10845;
    let t10851 = -t683 * t686 * t10589 / 64.0 - t683 * t686 * t10594 / 32.0 - t683 * t686 * t10599 / 64.0 - t10604 / 96.0 - t10607 / 96.0 - t683 * t686 * t10609 / 32.0 + t683 * t3167 * t10613 / 16.0 - t683 * t686 * t10617 / 64.0 - t683 * t686 * t10622 / 64.0 - t2035 * t2038 * t10626 / 48.0 - t8561 + t6492 / 96.0 - 3.0 / 64.0 * t136 * t10846 - 3.0 / 32.0 * t676 * t4073;
    (t10846, t10851)
}
