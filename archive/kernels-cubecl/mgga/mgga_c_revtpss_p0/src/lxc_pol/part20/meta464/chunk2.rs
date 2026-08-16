//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1766/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1766<F: Float>(t46654: F, t46714: F, t46782: F, t46848: F, t46911: F, t47212: F, t47279: F, t47340: F, t10111: F, t22: F, t4092: F, t39515: F, t4083: F) -> (F, F, F) {
    let t47343 = t46654 + t46714 + t46782 + t46848 + t46911 + t47212 + t47279 + t47340;
    let t47348 = t10111 * t4092 * t22;
    let t47351 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t4083;
    (t47343, t47348, t47351)
}
