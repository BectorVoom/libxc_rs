//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1030/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1030<F: Float>(t874: F, t8794: F, t352: F, t25820: F, t38977: F, t27101: F, t38980: F, t25854: F, t38983: F, t36058: F, t6444: F, t9005: F) -> (F, F, F, F, F, F) {
    let t41483 = t874 * t8794;
    let t41484 = t41483 * t352;
    let t41488 = t25820 * t38977;
    let t41490 = t27101 * t38980;
    let t41492 = t25854 * t38983;
    let t41500 = F::new(0.2927036860455597649e0) * t36058;
    let t41501 = t6444 * t9005;
    (t41484, t41488, t41490, t41492, t41500, t41501)
}
