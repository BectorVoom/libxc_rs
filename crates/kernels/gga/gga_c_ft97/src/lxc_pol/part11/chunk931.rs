//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 931/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk931<F: Float>(t1882: F, t9754: F, t9746: F, t675: F, t9568: F, t713: F, t9572: F, t446: F, t41433: F, t41437: F, t41439: F, t41443: F, t41797: F, t41801: F, t41803: F, t41807: F, t41808: F, t41810: F) -> (F, F, F, F, F) {
    let t41812 = t1882 * t9754;
    let t41814 = t1882 * t9746;
    let t41816 = t9568 * t675;
    let t41817 = t9572 * t713;
    let t41819 = t446 * t41816 * t41817;
    let t41821 = -4.0 / 3.0 * t41433 + 4.0 / 9.0 * t41437 + 4.0 / 9.0 * t41439 - 4.0 / 3.0 * t41443 - t41797 / 6.0 - t41801 + 2.0 / 9.0 * t41803 + t41807 - 2.0 / 9.0 * t41808 - 4.0 / 9.0 * t41810 + 4.0 / 27.0 * t41812 - 4.0 / 27.0 * t41814 + 20.0 / 81.0 * t41819;
    (t41812, t41814, t41817, t41819, t41821)
}
