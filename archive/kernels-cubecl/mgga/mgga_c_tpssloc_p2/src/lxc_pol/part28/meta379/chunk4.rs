//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1451/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1451<F: Float>(t1118: F, t14913: F, t1099: F, t14720: F, t14722: F, t14704: F, t11136: F, t11137: F, t11139: F, t11141: F, t11143: F, t14702: F, t14708: F, t14728: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F) -> (F, F) {
    let t14914 = t14913 * t1118;
    let t14916 = F::cast_from(1.0_f64) * t1099 * t14914;
    let t14922 = F::cast_from(0.41203703703703703704e-2_f64) * t14720;
    let t14923 = F::cast_from(0.12361111111111111111e-1_f64) * t14722;
    let t14924 = F::cast_from(0.61805555555555555556e-2_f64) * t14704;
    let t14933 = -t11136 + F::cast_from(0.82407407407407407407e-2_f64) * t11137 + F::cast_from(0.20601851851851851852e-2_f64) * t11139 - F::cast_from(0.61805555555555555556e-2_f64) * t11141 - F::cast_from(0.30902777777777777778e-2_f64) * t11143 + F::cast_from(0.41203703703703703704e-2_f64) * t14702 + t14922 - t14923 - t14924 + F::cast_from(0.10300925925925925926e-1_f64) * t14728 - F::cast_from(0.37083333333333333333e-1_f64) * t14733 - F::cast_from(0.12361111111111111111e-1_f64) * t14738 - F::cast_from(0.61805555555555555555e-2_f64) * t14742 + F::cast_from(0.55625000000000000001e-1_f64) * t14746 + F::cast_from(0.37083333333333333334e-1_f64) * t14751 + F::cast_from(0.18541666666666666667e-1_f64) * t14755 + F::cast_from(0.92708333333333333333e-2_f64) * t14708;
    (t14916, t14933)
}
