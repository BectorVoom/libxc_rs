//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1160/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1160<F: Float>(t1057: F, t7482: F, t6175: F, t7492: F, t1090: F, t2715: F, t2785: F, t221: F, t2627: F, t2807: F, t1101: F, t2626: F, t7509: F, t7737: F, t7743: F, t23653: F, t7612: F, t7658: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24088 = t7482 * t1057;
    let t24090 = t6175 * t1057;
    let t24095 = t7492 * t1057;
    let t24097 = t7492 * t1090;
    let t24099 = t2715 * t2785;
    let t24104 = t2807 * t221 * t2627;
    let t24108 = 0.67471172535210825684e-1 * t2626 * t7509 * t1101;
    let t24109 = t7737 * t7743;
    let t24113 = 0.19263893255070628431e1 * t23653 * t7658 * t7612;
    (t24088, t24090, t24095, t24097, t24099, t24104, t24108, t24109, t24113)
}
