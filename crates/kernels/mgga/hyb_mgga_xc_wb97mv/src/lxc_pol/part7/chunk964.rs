//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 964/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk964<F: Float>(t7190: F, t7192: F, t7195: F, t9271: F, t9275: F, t9292: F, t359: F, t3621: F, t7231: F, t2594: F, t3596: F, t3622: F, t1427: F, t7258: F, t2598: F, t2572: F) -> (F, F, F, F, F, F) {
    let t9294 = -t7190 + 0.47488888888888888888e-1 * t7192 - 0.17808333333333333333e-1 * t7195 + 0.23744444444444444444e-1 * t9271 - t9275 + 0.53425e-1 * t9292;
    let t9296 = 0.621814e-1 * t9294 * t359;
    let t9297 = t3621 * t7231;
    let t9300 = t2594 * t3596;
    let t9301 = t9300 * t3622;
    let t9304 = t7258 * t1427;
    let t9305 = t9304 * t2598;
    let t9310 = t2572 * t3596;
    (t9294, t9296, t9297, t9301, t9305, t9310)
}
