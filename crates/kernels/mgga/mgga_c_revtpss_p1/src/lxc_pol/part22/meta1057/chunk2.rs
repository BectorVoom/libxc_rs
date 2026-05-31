//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3749/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3749<F: Float>(t19661: F, t5405: F, t17241: F, t5373: F, t17654: F, t20766: F, t56756: F, t12809: F, t16696: F, t17247: F, t17250: F, t17429: F, t17476: F, t17651: F, t17693: F, t20800: F, t20806: F, t21213: F, t3689: F, t3694: F, t3720: F, t57660: F, t58899: F, t58975: F, t58997: F) -> (F, F) {
    let t71314 = t19661 * t5405;
    let t71320 = t5373 * t17241;
    let t71329 = t17654 * t56756 * t20766;
    let t71334 = F::cast_from(0.42874018118069736972e-3_f64) * t12809 * t3720 * t20800 * t16696 - F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t21213 * t3689 - F::cast_from(11.0_f64) / F::cast_from(162.0_f64) * t21213 * t3694 - F::cast_from(0.11433071498151929859e-2_f64) * t58975 + F::cast_from(0.28582678745379824648e-2_f64) * t17693 * t58899 * t71314 - F::cast_from(0.30488190661738479624e-2_f64) * t57660 * t17651 + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t71320 + t5373 * t17247 / F::cast_from(27.0_f64) + t5373 * t17250 / F::cast_from(9.0_f64) + F::cast_from(14.0_f64) / F::cast_from(243.0_f64) * t5373 * t17476 - F::cast_from(0.76220476654346199061e-3_f64) * t71329 - F::cast_from(0.42874018118069736972e-3_f64) * t17429 * t20806 + F::cast_from(0.11433071498151929859e-2_f64) * t58997;
    (t71314, t71334)
}
