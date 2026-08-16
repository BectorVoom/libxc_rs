//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 992/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk992<F: Float>(t3718: F, t6553: F, t12148: F, t2355: F, t1339: F, t1537: F, t46849: F, t590: F, t1441: F, t493: F, t475: F) -> (F, F, F, F, F) {
    let t47790 = t6553 * t3718;
    let t47791 = t2355 * t12148;
    let t47794 = t1537 * t1339 * t46849 * t590;
    let t47800 = t1441 * t493 * t46849 * t590;
    let t47803 = t46849 * t475;
    (t47790, t47791, t47794, t47800, t47803)
}
