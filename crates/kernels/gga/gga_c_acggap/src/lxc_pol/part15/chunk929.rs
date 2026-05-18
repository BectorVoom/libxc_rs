//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 929/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk929<F: Float>(t31699: F, t7588: F, t2074: F, t30797: F, t7844: F, t1111: F, t1992: F, t30147: F, t7586: F, t1165: F, t30209: F, t3044: F, t604: F) -> (F, F, F, F, F) {
    let t31700 = t31699 * t7588;
    let t31702 = t30797 * t2074;
    let t31704 = t31699 * t7844;
    let t31708 = t30147 * t7586 * t1992 * t1111;
    let t31720 = t30209 * t1165 * t604 * t3044;
    (t31700, t31702, t31704, t31708, t31720)
}
