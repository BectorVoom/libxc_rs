//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1198/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1198<F: Float>(t3638: F, t7706: F, t2848: F, t2860: F, t1148: F, t7907: F, t1117: F, t7917: F, t1537: F, t2869: F, t2873: F, t297: F, t532: F, t1157: F, t24244: F, t516: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27854 = t3638 * t7706;
    let t27882 = t2860 * t2848;
    let t27886 = t1148 * t7907;
    let t27890 = t1117 * t7917;
    let t27899 = t1537 * t2869;
    let t27911 = t1537 * t2873;
    let t27964 = t532 * t297;
    let t27965 = t1157 * t27964;
    let t27976 = t516 * t24244;
    (t27854, t27882, t27886, t27890, t27899, t27911, t27964, t27965, t27976)
}
