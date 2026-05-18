//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1115/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1115<F: Float>(t3146: F, t4573: F, t2667: F, t3101: F, t5255: F, t5416: F, t1111: F, t5285: F, t530: F, t3151: F, t26910: F, t5328: F) -> (F, F, F, F, F, F) {
    let t46469 = t3146 * t4573;
    let t46536 = t3101 * t5255 * t2667;
    let t46539 = t5416 * t2667;
    let t46590 = t1111 * t530 * t5285;
    let t46697 = t3151 * t4573;
    let t46715 = t26910 * t5328;
    (t46469, t46536, t46539, t46590, t46697, t46715)
}
