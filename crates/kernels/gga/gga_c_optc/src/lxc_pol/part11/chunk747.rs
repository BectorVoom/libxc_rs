//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 747/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk747<F: Float>(t131: F, t634: F, t9599: F, t1278: F, t2080: F, t160: F, t2086: F, t130: F, t2029: F, t1245: F, t2042: F, t2045: F) -> (F, F, F, F, F, F) {
    let t9641 = t634 * t131;
    let t9642 = t9641 * t9599;
    let t9651 = t2080 * t1278;
    let t9677 = t160 * t2086;
    let t9678 = t9641 * t9677;
    let t9686 = t130 * t2029;
    let t9701 = t2042 * t1245;
    let t9703 = t2045 * t1245;
    (t9642, t9651, t9678, t9686, t9701, t9703)
}
