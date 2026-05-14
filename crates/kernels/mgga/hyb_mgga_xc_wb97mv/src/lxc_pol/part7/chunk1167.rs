//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1167/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1167<F: Float>(t1175: F, t6465: F, t546: F, t8184: F, t10: F, t16: F, t6811: F, t2968: F, t2995: F, t554: F, t6432: F, t2999: F, t22469: F, t24: F, t3005: F, t556: F) -> (F, F, F, F, F, F, F, F) {
    let t25289 = t1175 * t6465;
    let t25291 = t546 * t8184;
    let t25294 = t6811 * t10 * t16;
    let t25295 = t25294 * t2968;
    let t25298 = t554 * t6432 * t2995;
    let t25301 = t554 * t6432 * t2999;
    let t25303 = t24 * t22469;
    let t25306 = t554 * t25303 * t556 * t3005;
    (t25289, t25291, t25294, t25295, t25298, t25301, t25303, t25306)
}
