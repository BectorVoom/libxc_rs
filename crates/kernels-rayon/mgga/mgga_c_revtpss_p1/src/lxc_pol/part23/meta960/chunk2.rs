//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3235/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3235(t1469: f64, t1486: f64, t72: f64, t1494: f64, t18281: f64, t1927: f64, t21686: f64, t21687: f64, t21727: f64, t22662: f64, t22672: f64, t22739: f64, t36: f64, t4186: f64, t4196: f64, t5825: f64, t5869: f64, t608: f64, t60823: f64, t627: f64, t6977: f64, t70: f64, t76397: f64, t7719: f64, t78770: f64, t85: f64) -> f64 {
    let t85161 = t1469 * t1486 * t72;
    let t85177 = -t21727 * t1494 / 4.0_f64 - t4196 * t5869 / 4.0_f64 - t608 * t22739 / 12.0_f64 - t4186 * t70 * t72 * t22662 / 4.0_f64 - t60823 * t22662 / 4.0_f64 - t21686 * t6977 * t5825 / 4.0_f64 - t21686 * t1927 * t18281 / 4.0_f64 - t85161 * t21687 / 2.0_f64 - t21686 * t7719 * t4186 / 2.0_f64 - t78770 * t70 * t85 / 12.0_f64 - t36 * t76397 * t70 * t85 / 12.0_f64 - t22672 * t627 * t85 / 12.0_f64;
    t85177
}
