//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 887/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk887<F: Float>(t352: F, t8700: F, t262: F, t7192: F, t2157: F, t5011: F, t2350: F, t866: F, t848: F, t8630: F, t833: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39674 = t8700 * t352;
    let t39675 = t262 * t39674;
    let t39676 = t7192 * t39675;
    let t39678 = t5011 * t2157;
    let t39680 = t2350 * t866;
    let t39681 = t262 * t39680;
    let t39682 = t7192 * t39681;
    let t39684 = t2350 * t848;
    let t39685 = t262 * t39684;
    let t39686 = t8630 * t39685;
    let t39688 = t2350 * t833;
    let t39689 = t262 * t39688;
    (t39674, t39675, t39676, t39678, t39680, t39681, t39682, t39684, t39685, t39686, t39688, t39689)
}
