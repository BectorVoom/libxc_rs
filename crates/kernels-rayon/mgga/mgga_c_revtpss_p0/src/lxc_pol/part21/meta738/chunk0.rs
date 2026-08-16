//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2591/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2591(t2435: f64, t9635: f64, t9590: f64, t9593: f64, t10179: f64, t1450: f64, t4146: f64, t1455: f64, t5808: f64, t46279: f64, t46281: f64, t46286: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47620 = t2435 * t9635;
    let t47638 = t9590 * t9593;
    let t47651 = t10179 * t1450;
    let t47671 = t4146 * t4146;
    let t47672 = 1.0_f64 / t47671;
    let t47730 = t1455 * t5808;
    let t47753 = 36.0_f64 * t46279;
    let t47754 = 180.0_f64 * t46281;
    let t47758 = 0.17544670867903938621e1_f64 * t46286;
    (t47620, t47638, t47651, t47672, t47730, t47753, t47754, t47758)
}
