//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2263/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2263(t46759: f64, t46784: f64, t46821: f64, t46858: f64, t225: f64, t13242: f64, t13244: f64, t13254: f64, t13265: f64, t13316: f64, t16836: f64, t237: f64, t249: f64, t2633: f64, t2643: f64, t2679: f64, t2684: f64, t41066: f64, t4178: f64, t4180: f64, t4181: f64, t46717: f64, t46733: f64, t46737: f64, t46742: f64, t46748: f64, t9629: f64, t9642: f64, t9958: f64) -> (f64, f64, f64) {
    let t46860 = t46759 + t46784 + t46821 + t46858;
    let t46861 = t46860 * t225;
    let t46868 = 7.0_f64 / 768.0_f64 * t46717 - t2643 * t4180 * t13242 * t2684 / 1024.0_f64 - t2643 * t4180 * t13242 * t2679 / 1024.0_f64 - t9642 * t13316 / 1024.0_f64 - t2643 * t4180 * t4181 * t9958 / 3072.0_f64 + 7.0_f64 / 768.0_f64 * t46733 - t16836 * t9629 / 128.0_f64 - 3.0_f64 / 512.0_f64 * t46737 * t13265 + 7.0_f64 / 256.0_f64 * t46742 + 3.0_f64 / 512.0_f64 * t4178 * t4180 * t13242 * t2633 - 7.0_f64 / 256.0_f64 * t46748 + t46861 * t237 * t249 / 3072.0_f64 + t13254 * t13244 / 256.0_f64 + 35.0_f64 / 384.0_f64 * t41066;
    (t46860, t46861, t46868)
}
