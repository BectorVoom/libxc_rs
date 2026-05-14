//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1411/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1411<F: Float>(t21559: F, t964: F, t1596: F, t220: F, t32465: F, t109756: F, t115118: F, t115137: F, t115139: F, t115144: F, t115150: F, t115157: F, t1163: F, t18953: F, t19033: F, t32346: F, t32354: F, t32439: F, t32458: F, t32464: F, t32468: F, t33771: F, t33794: F, t33862: F, t33911: F, t33914: F, t33916: F, t33922: F, t33941: F, t9536: F, t9539: F) -> (F, F, F) {
    let t115162 = t964 * t21559;
    let t115165 = t115162 * t32465 * t220 * t1596;
    let t115168 = -0.34722222222222222222e-2 * t115118 * t9539 - 0.17361111111111111111e-2 * t33941 * t32346 - 0.35740740740740740742e-2 * t109756 * t33771 + 0.34722222222222222222e-2 * t32354 * t33911 + 0.69444444444444444444e-2 * t32354 * t33916 + 0.34722222222222222222e-2 * t33794 * t32468 + 0.34722222222222222222e-2 * t9536 * t32458 * t33862 * t1163 + t115137 + 0.34722222222222222222e-2 * t9536 * t115139 + 0.17361111111111111111e-2 * t9536 * t115144 - 0.20104166666666666667e-2 * t32439 * t115150 + 0.34722222222222222222e-2 * t9536 * t32464 * t33914 * t18953 + 0.13888888888888888889e-1 * t9536 * t33922 * t115157 * t19033 + 0.26805555555555555556e-2 * t32439 * t115165;
    (t115162, t115165, t115168)
}
