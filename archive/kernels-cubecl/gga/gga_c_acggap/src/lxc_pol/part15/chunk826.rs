//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 826/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk826<F: Float>(t598: F, t9577: F, t604: F, t6847: F, t1181: F, t2068: F, t157: F, t495: F, t524: F, t599: F, t7337: F, t6841: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9578 = t598 * t9577;
    let t9582 = t604 * t6847;
    let t9583 = t1181 * t9582;
    let t9584 = t2068 * t9583;
    let t9587 = t495 * t524 * t157;
    let t9588 = t599 * t9587;
    let t9589 = t1181 * t9588;
    let t9590 = t7337 * t9589;
    let t9592 = t604 * t6841;
    (t9578, t9582, t9583, t9584, t9587, t9588, t9589, t9590, t9592)
}
