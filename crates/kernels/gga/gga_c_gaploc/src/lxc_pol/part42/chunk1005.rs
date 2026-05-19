//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 1005/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk1005<F: Float>(t42256: F, t46645: F, t46646: F, t46654: F, t46658: F, t46662: F, t46668: F, t46672: F, t46674: F, t46683: F, t46688: F, t46691: F, t46696: F, t46699: F, t48121: F, t48140: F, t48141: F, t48154: F, t48157: F, t48160: F) -> F {
    let t50757 = -t46645 - F::cast_from(0.38342925953920749676e0_f64) * t46646 - t46654 - t46658 - t46662 - t46668 + t46672 + t46674 + F::cast_from(0.63904876589867916127e-1_f64) * t42256 + F::cast_from(0.10224780254378866581e1_f64) * t48121 - t46683 - t46688 + t46691 + t46696 + t48140 + t48141 - t46699 + F::cast_from(0.17041300423964777634e0_f64) * t48154 - F::cast_from(0.17875244975925213335e0_f64) * t48157 + F::cast_from(0.11916829983950142223e0_f64) * t48160;
    t50757
}
