//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1788/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1788<F: Float>(t1448: F, t1450: F, t198: F, t39483: F, t39520: F, t39528: F, t39531: F, t46961: F, t46963: F, t46965: F, t46968: F, t46970: F, t46972: F, t46974: F, t46976: F, t46978: F, t47468: F, t47518: F, t47566: F, t47622: F, t532: F, t9400: F) -> F {
    let t47632 = t198 * t532 * (t47468 + t47518 + t47566 + t47622) * t1450 + t46961 - t46963 - t46965 + t46968 + t46970 + t46972 - t39483 + t39520 + F::cast_from(24.0_f64) * t198 * t9400 * t1448 * t1450 - t39528 - t46974 + t39531 + t46976 - t46978;
    t47632
}
