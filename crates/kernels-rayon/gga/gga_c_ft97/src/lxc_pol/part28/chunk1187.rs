//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1187/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1187(t147: f64, t35238: f64, t5: f64, t1080: f64, t140469: f64, t147091: f64, t147132: f64, t147159: f64, t147195: f64, t147224: f64, t147614: f64, t149270: f64, t149301: f64, t149335: f64, t149369: f64, t149404: f64, t149432: f64, t149458: f64, t149491: f64, t149607: f64, t149630: f64, t184: f64, t21: f64, t33231: f64, t33234: f64, t35239: f64, t363: f64, t3660: f64, t3665: f64, t3668: f64, t3674: f64, t3678: f64, t650: f64, t920: f64) -> f64 {
    let t148 = 10000000.0_f64 <= t147;
    let t149639 = t5 * t35238;
    let t149661 = piecewise3(t148, 0.0_f64, t5 * (t147091 + t147132 + t147159 + t147195 + t147224 + t147614 + t149270 + t149301 + t149335 + t149369 + t149404 + t149432 + t149458 + t149491 + t149607 + t149630) * t184 * t21 / 4.0_f64 + t149639 * t650 / 4.0_f64 + t5 * t35239 * t363 / 4.0_f64 + t140469 * t1080 / 4.0_f64 + t33234 * t3660 / 4.0_f64 + t33234 * t3665 / 4.0_f64 + t33234 * t3668 / 4.0_f64 + t5 * t33231 * t920 / 4.0_f64 + t33234 * t3674 / 4.0_f64 + t33234 * t3678 / 2.0_f64);
    t149661
}
