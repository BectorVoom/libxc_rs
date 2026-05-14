//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 399/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk399<F: Float>(t1348: F, t1349: F, t2110: F, t2181: F, t2192: F, t2209: F, t338: F, t417: F, t451: F) -> (F,) {
    let t2211 = -t1348 - 0.23426533963880895498e-2 * t1349 * t2181 - 0.46853067927761790996e-2 * t417 * t2192 - t2110 * t451 - t338 * t2209;
    (t2211,)
}
