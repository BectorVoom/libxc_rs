//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1052/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1052<F: Float>(t1422: F, t161: F, t1353: F, t2486: F, t4624: F, t1428: F, t4398: F, t197: F, t2293: F, t1: F, t20073: F, t493: F) -> (F, F, F, F, F, F, F) {
    let t20901 = t1422 * t161;
    let t20902 = t20901 * t1353;
    let t20954 = t4624 * t2486;
    let t20957 = t4398 * t1428;
    let t21004 = t197 * t2293;
    let t21005 = t21004 * t1;
    let t21042 = t493 * t20073;
    (t20901, t20902, t20954, t20957, t21004, t21005, t21042)
}
