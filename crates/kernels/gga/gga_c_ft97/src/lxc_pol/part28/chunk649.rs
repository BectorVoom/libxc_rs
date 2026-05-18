//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 649/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk649<F: Float>(t26045: F, t83: F, t25867: F, t23323: F, t3114: F, t3189: F, t1326: F, t1780: F, t3195: F, t23327: F, t3205: F, t103: F, t6454: F) -> (F, F, F, F, F, F, F) {
    let t26337 = t83 * t26045;
    let t26340 = t83 * t25867;
    let t26343 = t23323 * t3114;
    let t26346 = t23323 * t3189;
    let t26349 = t1780 * t1326;
    let t26350 = t26349 * t3195;
    let t26353 = t23327 * t3205;
    let t26356 = t103 * t6454;
    (t26337, t26340, t26343, t26346, t26350, t26353, t26356)
}
