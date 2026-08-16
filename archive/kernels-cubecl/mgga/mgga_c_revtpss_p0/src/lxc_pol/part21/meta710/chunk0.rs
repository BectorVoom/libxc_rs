//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2540/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2540<F: Float>(t550: F, t9898: F, t2661: F, t46609: F, t9994: F, t3992: F, t543: F, t9890: F, t3995: F, t40488: F, t3989: F, t9944: F) -> (F, F, F, F, F) {
    let t46610 = t550 * t9898;
    let t46613 = t2661 * t46609 * t46610 * t9994;
    let t46618 = t2661 * t3992 * t550 * t9890 * t543;
    let t46620 = t40488 * t3995;
    let t46622 = t3989 * t9944;
    (t46610, t46613, t46618, t46620, t46622)
}
