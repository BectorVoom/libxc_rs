//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3307/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3307<F: Float>(t22483: F, t22809: F, t39773: F, t4139: F, t4140: F, t46996: F, t46998: F, t47003: F, t48256: F, t48259: F, t48261: F, t5541: F, t5778: F, t85905: F, t85906: F) -> F {
    let t86751 = -F::cast_from(3.0_f64) * t22483 * t5541 * t5778 + F::cast_from(3.0_f64) * t22809 * t4139 * t4140 + t39773 + t46996 - t46998 + t47003 - t48256 - t48259 + t48261 - t85905 - t85906;
    t86751
}
