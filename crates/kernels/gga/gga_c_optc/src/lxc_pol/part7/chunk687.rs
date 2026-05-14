//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 687/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk687<F: Float>(t1972: F, t712: F, t171: F, t1974: F, t2045: F, t592: F, t2042: F, t559: F, t104: F, t188: F, t6465: F, t6741: F, t6744: F, t6747: F, t6750: F, t6753: F, t6757: F, t6761: F, t95: F) -> (F, F, F, F, F) {
    let t6763 = t1972 * t712;
    let t6766 = 1.0 / t1974 / t171;
    let t6770 = t2045 * t592;
    let t6771 = 36.0 * t6770;
    let t6772 = t2042 * t559;
    let t6773 = 60.0 * t6772;
    let t6774 = t6741 + t6744 - t6747 - t6750 + t6753 + t188 * t6757 / 2.0 - 7.0 / 2.0 * t6761 + t6465 + 0.51689762869806860992e-2 * t95 * t104 * t6763 * t6766 + t6771 + t6773;
    (t6763, t6766, t6771, t6773, t6774)
}
