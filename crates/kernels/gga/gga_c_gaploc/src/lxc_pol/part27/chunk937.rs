//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 937/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk937<F: Float>(t10006: F, t10995: F, t11010: F, t11011: F, t11012: F, t11015: F, t11018: F, t11024: F, t11028: F, t11032: F, t11041: F, t11043: F, t11046: F, t11049: F, t11056: F, t296: F, t3720: F) -> (F, F) {
    let t12249 = t10995 + t11010 + t10006 - t11011 + t11012 + t11015 - t11018 - t11024 - t11028 - t11032 - t11041 + t11043 + t11046 + t11049 - t11056;
    let t12250 = t296 * t3720;
    (t12249, t12250)
}
