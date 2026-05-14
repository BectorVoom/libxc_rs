//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 761/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk761<F: Float>(t2030: F, t9663: F, t2317: F, t507: F, t2060: F, t1849: F, t604: F, t1181: F, t7575: F, t1165: F, t1844: F, t2068: F, t1856: F, t2001: F, t1426: F, t368: F, t9536: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9664 = t2030 * t9663;
    let t9666 = t507 * t2317;
    let t9667 = t2060 * t9666;
    let t9669 = t604 * t1849;
    let t9670 = t1181 * t9669;
    let t9671 = t7575 * t9670;
    let t9674 = t1165 * t604 * t1844;
    let t9675 = t2068 * t9674;
    let t9677 = t2001 * t1856;
    let t9681 = t1426 * t368 * t9536;
    (t9664, t9666, t9667, t9669, t9670, t9671, t9674, t9675, t9677, t9681)
}
