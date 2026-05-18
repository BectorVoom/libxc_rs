//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 832/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk832<F: Float>(t1426: F, t368: F, t9536: F, t598: F, t1772: F, t6: F, t422: F, t599: F, t5679: F, t604: F, t1891: F, t2001: F) -> (F, F, F, F, F, F, F, F) {
    let t9681 = t1426 * t368 * t9536;
    let t9682 = t598 * t9681;
    let t9685 = t6 * t1772;
    let t9687 = t422 * t9685 * t599;
    let t9688 = t598 * t9687;
    let t9691 = t422 * t5679 * t604;
    let t9692 = t598 * t9691;
    let t9694 = t2001 * t1891;
    (t9681, t9682, t9685, t9687, t9688, t9691, t9692, t9694)
}
