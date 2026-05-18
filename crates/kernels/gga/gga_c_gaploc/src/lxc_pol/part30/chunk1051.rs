//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1051/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1051<F: Float>(t20369: F, t4130: F, t20539: F, t493: F, t4803: F, t6575: F, t4786: F, t6582: F, t1406: F, t6715: F, t1339: F, t20117: F) -> (F, F, F, F, F, F) {
    let t21154 = t4130 * t20369;
    let t21172 = t493 * t20539;
    let t21272 = t4803 * t6575;
    let t21283 = t4786 * t6582;
    let t21370 = t1406 * t6715;
    let t21389 = t1339 * t20117;
    (t21154, t21172, t21272, t21283, t21370, t21389)
}
