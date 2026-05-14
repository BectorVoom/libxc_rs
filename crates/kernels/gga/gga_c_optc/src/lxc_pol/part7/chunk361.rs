//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 361/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk361<F: Float>(t463: F, t27: F, t513: F, t13: F, t533: F, sigma2: F) -> (F, F, F, F, F) {
    let t1724 = t463 * sigma2;
    let t1755 = t513 * t27;
    let t1756 = 1.0 / t1755;
    let t1757 = t13 * t1756;
    let t1758 = t533 * t533;
    (t1724, t1755, t1756, t1757, t1758)
}
