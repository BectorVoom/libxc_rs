//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1483/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1483<F: Float>(t22449: F, t2435: F, t136: F, t2457: F, t6918: F, t9674: F, t124: F, t6861: F, t46917: F, t6871: F, t22102: F, t46740: F) -> (F, F, F, F, F) {
    let t73707 = t2435 * t22449;
    let t73712 = t9674 * t6918 * t136 * t2457;
    let t73731 = t124 * t6861;
    let t73778 = t46917 * t6871;
    let t73789 = t46740 * t22102;
    (t73707, t73712, t73731, t73778, t73789)
}
