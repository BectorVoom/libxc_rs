//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 332/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk332<F: Float>(t1403: F, t1404: F, t1924: F, t1951: F, t1962: F, t1979: F, t486: F, t510: F, t538: F) -> (F,) {
    let t1981 = -t1403 - 0.23426533963880895498e-2 * t1404 * t1951 - 0.46853067927761790996e-2 * t510 * t1962 - t1924 * t538 - t486 * t1979;
    (t1981,)
}
