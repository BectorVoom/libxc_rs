//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 630/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk630<F: Float>(t1328: F, t197: F, t1: F, t544: F, t594: F, t106: F) -> (F, F, F, F) {
    let t4382 = t197 * t1328;
    let t4383 = t4382 * t1;
    let t4384 = t544 * t4383;
    let t4389 = t594 * t1;
    let t4390 = t4389 * t106;
    (t4382, t4384, t4389, t4390)
}
