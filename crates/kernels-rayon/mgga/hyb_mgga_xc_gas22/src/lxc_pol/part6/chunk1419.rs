//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1419/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1419(t11282: f64, t7785: f64, t11315: f64, t1161: f64, t22531: f64, t2829: f64, t2869: f64, t30571: f64, t30682: f64, t30686: f64, t30689: f64, t30692: f64, t30697: f64, t30703: f64, t30710: f64, t3739: f64, t3747: f64, t4512: f64, t7637: f64, t7643: f64, t7800: f64, t7806: f64, t9587: f64, t9594: f64, t9657: f64) -> (f64, f64) {
    let t30716 = t11282 * t7785;
    let t30719 = 512.0_f64 / 81.0_f64 * t30682 * t9657 + 5632.0_f64 / 2187.0_f64 * t9587 * t30686 + 704.0_f64 / 81.0_f64 * t3747 * t30689 + 1408.0_f64 / 243.0_f64 * t3739 * t30692 + 5632.0_f64 / 2187.0_f64 * t9594 * t30686 + 128.0_f64 / 3.0_f64 * t7806 * t30697 + 256.0_f64 / 81.0_f64 * t22531 * t30571 + 616.0_f64 / 9.0_f64 * t7637 * t30703 + 440.0_f64 / 9.0_f64 * t7800 * t1161 * t4512 * t2869 + 88.0_f64 / 9.0_f64 * t7643 * t30710 + 440.0_f64 / 9.0_f64 * t7800 * t11315 * t7785 - 88.0_f64 / 27.0_f64 * t2829 * t30716;
    (t30716, t30719)
}
