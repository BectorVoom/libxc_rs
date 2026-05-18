//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1047/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1047<F: Float>(t41806: F, t1882: F, t9758: F, t9741: F, t9754: F, t9746: F, t675: F, t9568: F, t713: F, t9572: F, t446: F, t41433: F, t41437: F, t41439: F, t41443: F, t41797: F, t41801: F, t41803: F) -> (F, F, F, F, F, F, F) {
    let t41807 = F::new(56.0) / F::new(81.0) * t41806;
    let t41808 = t1882 * t9758;
    let t41810 = t1882 * t9741;
    let t41812 = t1882 * t9754;
    let t41814 = t1882 * t9746;
    let t41816 = t9568 * t675;
    let t41817 = t9572 * t713;
    let t41819 = t446 * t41816 * t41817;
    let t41821 = -F::new(4.0) / F::new(3.0) * t41433 + F::new(4.0) / F::new(9.0) * t41437 + F::new(4.0) / F::new(9.0) * t41439 - F::new(4.0) / F::new(3.0) * t41443 - t41797 / F::new(6.0) - t41801 + F::new(2.0) / F::new(9.0) * t41803 + t41807 - F::new(2.0) / F::new(9.0) * t41808 - F::new(4.0) / F::new(9.0) * t41810 + F::new(4.0) / F::new(27.0) * t41812 - F::new(4.0) / F::new(27.0) * t41814 + F::new(20.0) / F::new(81.0) * t41819;
    (t41808, t41810, t41812, t41814, t41817, t41819, t41821)
}
