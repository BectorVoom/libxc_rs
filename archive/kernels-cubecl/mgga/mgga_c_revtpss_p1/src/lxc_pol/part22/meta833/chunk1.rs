//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2957/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2957<F: Float>(t13824: F, t221: F, t3978: F, t46716: F, t13923: F, t3930: F, t14036: F, t9976: F, t46694: F, t5686: F, t14030: F, t9744: F) -> (F, F, F, F, F) {
    let t48662 = t221 * t13824;
    let t48664 = t3978 * t46716 * t48662;
    let t48666 = t3930 * t13923;
    let t48668 = t9976 * t14036;
    let t48685 = t46694 * t5686;
    let t48687 = t9744 * t14030;
    (t48664, t48666, t48668, t48685, t48687)
}
