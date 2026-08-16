//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 390/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk390<F: Float>(t1875: F, t612: F, t583: F, t618: F, t617: F, t122: F, t653: F, t128: F) -> (F, F, F, F, F) {
    let t1876 = t1875 * t612;
    let t1877 = t618 * t583;
    let t1878 = t617 * t1877;
    let t1881 = t122 * t653;
    let t1882 = t1881 * t128;
    (t1876, t1877, t1878, t1881, t1882)
}
