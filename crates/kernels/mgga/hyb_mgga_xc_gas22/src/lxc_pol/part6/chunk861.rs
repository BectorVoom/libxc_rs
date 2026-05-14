//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 861/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk861<F: Float>(t2631: F, t7497: F, t1057: F, t2814: F, t1068: F, t2751: F, t1100: F, t2696: F, t462: F, t10: F, t1107: F, t1095: F, t2639: F, t2635: F, t2674: F, t1110: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7498 = t7497 * t2631;
    let t7503 = 12.0 * t1057 * t2814;
    let t7506 = t2751 * t1068;
    let t7508 = t2696 * t1100;
    let t7509 = t462 * t7508;
    let t7511 = t2696 * t10;
    let t7512 = t7511 * t1107;
    let t7515 = t2639 * t1095;
    let t7516 = t2635 * t2674 * t7515;
    let t7518 = 0.51947577317044391277e2 * t1110 * t7516;
    (t7498, t7503, t7506, t7508, t7509, t7511, t7512, t7515, t7516, t7518)
}
