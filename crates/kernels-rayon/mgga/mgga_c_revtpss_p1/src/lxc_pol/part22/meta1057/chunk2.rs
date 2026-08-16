//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3749/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3749(t19661: f64, t5405: f64, t17241: f64, t5373: f64, t17654: f64, t20766: f64, t56756: f64, t12809: f64, t16696: f64, t17247: f64, t17250: f64, t17429: f64, t17476: f64, t17651: f64, t17693: f64, t20800: f64, t20806: f64, t21213: f64, t3689: f64, t3694: f64, t3720: f64, t57660: f64, t58899: f64, t58975: f64, t58997: f64) -> (f64, f64) {
    let t71314 = t19661 * t5405;
    let t71320 = t5373 * t17241;
    let t71329 = t17654 * t56756 * t20766;
    let t71334 = 0.42874018118069736972e-3_f64 * t12809 * t3720 * t20800 * t16696 - 11.0_f64 / 324.0_f64 * t21213 * t3689 - 11.0_f64 / 162.0_f64 * t21213 * t3694 - 0.11433071498151929859e-2_f64 * t58975 + 0.28582678745379824648e-2_f64 * t17693 * t58899 * t71314 - 0.30488190661738479624e-2_f64 * t57660 * t17651 + 2.0_f64 / 81.0_f64 * t71320 + t5373 * t17247 / 27.0_f64 + t5373 * t17250 / 9.0_f64 + 14.0_f64 / 243.0_f64 * t5373 * t17476 - 0.76220476654346199061e-3_f64 * t71329 - 0.42874018118069736972e-3_f64 * t17429 * t20806 + 0.11433071498151929859e-2_f64 * t58997;
    (t71314, t71334)
}
