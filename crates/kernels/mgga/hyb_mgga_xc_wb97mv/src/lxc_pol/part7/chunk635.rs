//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 635/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk635<F: Float>(t1187: F, t54: F, t3046: F, t587: F, t57: F, t591: F, t60: F, t595: F, t63: F, t599: F, t66: F, t603: F, t69: F, t607: F, t1911: F, t611: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3049 = t54 * t1187;
    let t3052 = t587 * t3046;
    let t3054 = t57 * t1187;
    let t3057 = t591 * t3046;
    let t3059 = t60 * t1187;
    let t3062 = t595 * t3046;
    let t3064 = t63 * t1187;
    let t3067 = t599 * t3046;
    let t3069 = t66 * t1187;
    let t3072 = t603 * t3046;
    let t3074 = t69 * t1187;
    let t3077 = t607 * t3046;
    let t3079 = t1911 * t1187;
    let t3082 = t611 * t3046;
    (t3049, t3052, t3054, t3057, t3059, t3062, t3064, t3067, t3069, t3072, t3074, t3077, t3079, t3082)
}
