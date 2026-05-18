//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 334/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk334<F: Float>(t1453: F, t6: F, t101: F, t118: F, t136: F) -> (F, F, F, F) {
    let t1454 = t6 * t1453;
    let t1455 = t1454 * t101;
    let t1456 = t136 * t118;
    let t1457 = F::new(1.0) / t1456;
    (t1454, t1455, t1456, t1457)
}
