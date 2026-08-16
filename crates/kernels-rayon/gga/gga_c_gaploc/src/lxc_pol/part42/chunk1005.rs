//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 1005/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk1005(t42256: f64, t46645: f64, t46646: f64, t46654: f64, t46658: f64, t46662: f64, t46668: f64, t46672: f64, t46674: f64, t46683: f64, t46688: f64, t46691: f64, t46696: f64, t46699: f64, t48121: f64, t48140: f64, t48141: f64, t48154: f64, t48157: f64, t48160: f64) -> f64 {
    let t50757 = -t46645 - 0.38342925953920749676e0_f64 * t46646 - t46654 - t46658 - t46662 - t46668 + t46672 + t46674 + 0.63904876589867916127e-1_f64 * t42256 + 0.10224780254378866581e1_f64 * t48121 - t46683 - t46688 + t46691 + t46696 + t48140 + t48141 - t46699 + 0.17041300423964777634e0_f64 * t48154 - 0.17875244975925213335e0_f64 * t48157 + 0.11916829983950142223e0_f64 * t48160;
    t50757
}
