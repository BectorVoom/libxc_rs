//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 901/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk901<F: Float>(t16979: F, t3813: F, t14390: F, t2643: F, t4772: F, t3634: F, t1382: F, t4941: F) -> (F, F, F, F, F) {
    let t16980 = t16979 * t3813;
    let t16981 = t14390 * t16980;
    let t16984 = t2643 * t4772;
    let t16985 = t3634 * t16984;
    let t16988 = t4941 * t1382;
    (t16980, t16981, t16984, t16985, t16988)
}
