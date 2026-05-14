//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 765/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk765<F: Float>(t1426: F, t368: F, t9536: F, t598: F, t1772: F, t6: F, t422: F, t599: F, t5679: F, t604: F, t1891: F, t2001: F, t1896: F, t1901: F, t1734: F, t142: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9681 = t1426 * t368 * t9536;
    let t9682 = t598 * t9681;
    let t9685 = t6 * t1772;
    let t9687 = t422 * t9685 * t599;
    let t9688 = t598 * t9687;
    let t9691 = t422 * t5679 * t604;
    let t9692 = t598 * t9691;
    let t9694 = t2001 * t1891;
    let t9696 = t2001 * t1896;
    let t9698 = t2001 * t1901;
    let t9700 = t599 * t1734;
    let t9701 = t142 * t9700;
    (t9681, t9682, t9685, t9687, t9688, t9691, t9692, t9694, t9696, t9698, t9700, t9701)
}
