//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1439/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1439<F: Float>(t115645: F, t9536: F, t1310: F, t4375: F, t32441: F, t36521: F, t114796: F, t109626: F, t109633: F, t110092: F, t110097: F, t110099: F, t110106: F, t114794: F, t115669: F, t115851: F, t115895: F, t32339: F, t32439: F, t32461: F, t32468: F, t33771: F, t33941: F) -> (F,) {
    let t115950 = 0.11574074074074074074e-2 * t9536 * t115645;
    let t115955 = t1310 * t4375;
    let t115957 = t115955 * t36521 * t32441;
    let t115969 = 0.15476481481481481481e-2 * t114796;
    let t115970 = 0.61905925925925925926e-2 * t110092 - 0.61905925925925925926e-2 * t110097 + 0.11349419753086419753e-1 * t110099 - 0.51588271604938271604e-3 * t110106 - 0.92592592592592592593e-2 * t32339 * t33771 + t115950 + 0.34722222222222222222e-2 * t33941 * t32461 + 0.34722222222222222222e-2 * t33941 * t32468 + 0.20833333333333333334e-1 * t9536 * t115957 + 0.80416666666666666669e-2 * t32439 * t115957 - 0.69444444444444444444e-2 * t109626 * t115851 - 0.40208333333333333334e-2 * t109633 * t115669 - 0.13402777777777777778e-2 * t109633 * t115895 - 0.17411041666666666666e-2 * t114794 + t115969;
    (t115970,)
}
