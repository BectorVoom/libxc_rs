//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1050/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1050<F: Float>(t4803: F, t6582: F, t1535: F, t9419: F, t1433: F, t20395: F, t2366: F, t6519: F, t9439: F, t9448: F, t1359: F, t2293: F) -> (F, F, F, F, F, F, F) {
    let t20675 = t4803 * t6582;
    let t20687 = t1535 * t9419;
    let t20688 = t1433 * t20687;
    let t20692 = t2366 * t20395;
    let t20696 = t9439 * t6519;
    let t20700 = t9448 * t6519;
    let t20731 = t1359 * t2293;
    (t20675, t20687, t20688, t20692, t20696, t20700, t20731)
}
