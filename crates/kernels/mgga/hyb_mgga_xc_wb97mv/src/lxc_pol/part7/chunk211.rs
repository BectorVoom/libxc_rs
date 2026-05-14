//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 211/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk211<F: Float>(t43: F, t564: F, t583: F, t587: F, t591: F, t595: F, t599: F, t603: F, t607: F, t611: F, t615: F, t634: F, t72: F, t88: F) -> (F,) {
    let t44 = 0.135e1 <= t43;
    let t638 = piecewise3(t44, -t564 * t583 / 18.0 + t587 * t583 / 240.0 - t591 * t583 / 4480.0 + t595 * t583 / 103680.0 - t599 * t583 / 2838528.0 + t603 * t583 / 89456640.0 - t607 * t583 / 0.31850496e10 + t611 * t583 / 0.1263403008e12, -8.0 / 3.0 * t615 * t88 - 8.0 / 3.0 * t72 * t634);
    (t638,)
}
