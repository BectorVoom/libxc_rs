//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 754/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk754<F: Float>(t143: F, t1264: F, t1279: F, t172: F, t187: F, t4025: F, t4026: F, t4062: F, t139: F, t214: F, t26: F, t1284: F, t1312: F, t196: F, t399: F) -> (F, F, F, F, F, F) {
    let t144 = 0.135e1 <= t143;
    let t4066 = piecewise3(t144, t4025, -8.0 / 3.0 * t4026 * t187 - 16.0 / 3.0 * t1264 * t1279 - 8.0 / 3.0 * t172 * t4062);
    let t4067 = t139 * t4066;
    let t4068 = t4067 * t214;
    let t4069 = t26 * t4068;
    let t4072 = t1284 * t1312;
    let t4073 = t26 * t4072;
    let t4077 = 1.0 / t196 / t399;
    (t4066, t4068, t4069, t4072, t4073, t4077)
}
