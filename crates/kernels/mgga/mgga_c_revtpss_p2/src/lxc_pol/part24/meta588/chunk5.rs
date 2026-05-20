//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1841/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1841<F: Float>(t1450: F, t1907: F, t198: F, t22483: F, t22809: F, t22813: F, t30122: F, t39747: F, t39750: F, t39756: F, t39760: F, t4139: F, t46980: F, t46988: F, t46992: F, t46996: F, t46998: F, t47000: F, t47003: F, t5532: F, t91963: F) -> F {
    let t92465 = F::new(24.0) * t1450 * t1907 * t198 * t22813 - F::new(36.0) * t22483 * t30122 * t4139 + F::new(12.0) * t22809 * t4139 * t5532 + t39747 + t39750 + t39756 + t39760 + t46980 + t46988 + t46992 + t46996 - t46998 - t47000 + t47003 + t91963;
    t92465
}
