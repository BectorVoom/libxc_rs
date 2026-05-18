//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1221/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1221<F: Float>(t2629: F, t530: F, t862: F, t2634: F, t24: F, t7406: F, t2623: F, t7917: F, t7914: F, t2640: F, t7468: F, t7477: F) -> (F, F, F, F, F, F) {
    let t25194 = t862 * t530 * t2629;
    let t25197 = t862 * t530 * t2634;
    let t25200 = t862 * t24 * t7406;
    let t25202 = t2623 * t7917;
    let t25208 = t2623 * t7914;
    let t25215 = t2640 * t7468 * t7477;
    (t25194, t25197, t25200, t25202, t25208, t25215)
}
