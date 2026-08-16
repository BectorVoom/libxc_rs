//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1205/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1205(t1548: f64, t16752: f64, t17691: f64, t573: f64, t16609: f64, t6028: f64, t7948: f64, t1542: f64, t27543: f64, t5905: f64, t97661: f64, t97663: f64, t97665: f64, t97667: f64, t97669: f64, t97671: f64, t97673: f64, t97675: f64, t97677: f64, t97679: f64, t97682: f64, t97684: f64, t97686: f64, t97688: f64) -> (f64, f64, f64, f64, f64) {
    let t97690 = t16752 * t1548;
    let t97692 = t17691 * t573;
    let t97695 = t7948 * t6028 * t16609;
    let t97698 = t1542 * t27543 * t5905;
    let t97700 = -0.20234375e-1_f64 * t97661 - 0.53958333333333333334e-1_f64 * t97663 - 0.26979166666666666667e-1_f64 * t97665 + 0.14388888888888888889e0_f64 * t97667 + 0.47962962962962962963e-1_f64 * t97669 + 0.20234375e-1_f64 * t97671 + 0.375e0_f64 * t97673 - 0.89930555555555555557e-2_f64 * t97675 + 0.20833333333333333333e-1_f64 * t97677 - 0.26979166666666666667e-1_f64 * t97679 + 0.1875e0_f64 * t97682 + 0.1875e0_f64 * t97684 - 0.16666666666666666667e0_f64 * t97686 + 0.375e0_f64 * t97688 + 0.4046875e-1_f64 * t97690 + 0.9375e-1_f64 * t97692 - 0.9375e-1_f64 * t97695 - 0.17986111111111111111e-1_f64 * t97698;
    (t97690, t97692, t97695, t97698, t97700)
}
