//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 758/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk758<F: Float>(t157: F, t495: F, t524: F, t599: F, t1181: F, t7337: F, t604: F, t6841: F, t2068: F, t1165: F, t7351: F, t1854: F, t7564: F, t1750: F, t7561: F, t1713: F, t579: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9587 = t495 * t524 * t157;
    let t9588 = t599 * t9587;
    let t9589 = t1181 * t9588;
    let t9590 = t7337 * t9589;
    let t9592 = t604 * t6841;
    let t9593 = t1181 * t9592;
    let t9594 = t2068 * t9593;
    let t9597 = t1165 * t604 * t9587;
    let t9598 = t7337 * t9597;
    let t9601 = t1165 * t7351 * t6841;
    let t9602 = t2068 * t9601;
    let t9607 = t7351 * t1854;
    let t9608 = t1181 * t9607;
    let t9609 = t7564 * t9608;
    let t9611 = t7561 * t1750;
    let t9613 = t579 * t1713;
    (t9587, t9588, t9589, t9590, t9592, t9593, t9594, t9597, t9598, t9601, t9602, t9607, t9608, t9609, t9611, t9613)
}
