//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1088/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1088<F: Float>(t1056: F, t3217: F, t7371: F, t2207: F, t22672: F, t2580: F, t474: F, t11613: F, t2786: F, t996: F, t11616: F, t3212: F, t10366: F, t3209: F, t11682: F, t23678: F, t2415: F, t2546: F) -> (F, F, F, F, F, F, F) {
    let t35999 = t3217 * t1056 * t7371;
    let t36003 = t22672 * t2207 * t474 * t2580;
    let t36006 = t996 * t2786 * t11613;
    let t36009 = t3212 * t11616;
    let t36011 = t10366 * t11613;
    let t36013 = t3209 * t11616;
    let t36017 = t11682 * t2415 * t2546 * t23678;
    (t35999, t36003, t36006, t36009, t36011, t36013, t36017)
}
