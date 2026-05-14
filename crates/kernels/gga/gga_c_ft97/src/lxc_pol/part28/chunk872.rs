//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 872/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk872<F: Float>(t1882: F, t33012: F, t33207: F, t7359: F, t8232: F, t33041: F, t8392: F, t7409: F, t33184: F, t7397: F, t33193: F, t604: F, t7339: F, t139320: F, t139323: F, t139492: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t139791 = t1882 * t33012;
    let t139808 = t1882 * t33207;
    let t139811 = 8.0 / 27.0 * t8232 * t7359;
    let t139820 = t8392 * t33041;
    let t139823 = 4.0 / 27.0 * t8232 * t7409;
    let t139888 = t1882 * t33184;
    let t139896 = 8.0 / 27.0 * t8232 * t7397;
    let t139940 = t8392 * t33193;
    let t139950 = t604 * t7339;
    let t139991 = 4.0 / 9.0 * t139320;
    let t139992 = 2.0 / 9.0 * t139323;
    let t140041 = 4.0 / 9.0 * t139492;
    (t139791, t139808, t139811, t139820, t139823, t139888, t139896, t139940, t139950, t139991, t139992, t140041)
}
