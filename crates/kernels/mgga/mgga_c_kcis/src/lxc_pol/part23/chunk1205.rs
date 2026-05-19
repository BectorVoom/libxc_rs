//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1205/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1205<F: Float>(t1548: F, t16752: F, t17691: F, t573: F, t16609: F, t6028: F, t7948: F, t1542: F, t27543: F, t5905: F, t97661: F, t97663: F, t97665: F, t97667: F, t97669: F, t97671: F, t97673: F, t97675: F, t97677: F, t97679: F, t97682: F, t97684: F, t97686: F, t97688: F) -> (F, F, F, F, F) {
    let t97690 = t16752 * t1548;
    let t97692 = t17691 * t573;
    let t97695 = t7948 * t6028 * t16609;
    let t97698 = t1542 * t27543 * t5905;
    let t97700 = -F::new(0.20234375e-1) * t97661 - F::cast_from(0.53958333333333333334e-1_f64) * t97663 - F::cast_from(0.26979166666666666667e-1_f64) * t97665 + F::cast_from(0.14388888888888888889e0_f64) * t97667 + F::cast_from(0.47962962962962962963e-1_f64) * t97669 + F::new(0.20234375e-1) * t97671 + F::new(0.375e0) * t97673 - F::cast_from(0.89930555555555555557e-2_f64) * t97675 + F::cast_from(0.20833333333333333333e-1_f64) * t97677 - F::cast_from(0.26979166666666666667e-1_f64) * t97679 + F::new(0.1875e0) * t97682 + F::new(0.1875e0) * t97684 - F::cast_from(0.16666666666666666667e0_f64) * t97686 + F::new(0.375e0) * t97688 + F::new(0.4046875e-1) * t97690 + F::new(0.9375e-1) * t97692 - F::new(0.9375e-1) * t97695 - F::cast_from(0.17986111111111111111e-1_f64) * t97698;
    (t97690, t97692, t97695, t97698, t97700)
}
