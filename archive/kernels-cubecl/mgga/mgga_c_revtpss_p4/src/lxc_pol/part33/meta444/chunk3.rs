//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1621/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1621<F: Float>(t12297: F, t12367: F, t16706: F, t16820: F, t16821: F, t16822: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> F {
    let t20567 = -t12367 + F::cast_from(0.41203703703703703703e-2_f64) * t12297 + F::cast_from(0.82407407407407407408e-2_f64) * t16706 + t16820 - t16821 - t16822 + F::cast_from(0.20601851851851851852e-2_f64) * t20283 + F::cast_from(0.10300925925925925926e-1_f64) * t20295 - F::cast_from(0.37083333333333333333e-1_f64) * t20300 - F::cast_from(0.12361111111111111111e-1_f64) * t20304 - F::cast_from(0.61805555555555555557e-2_f64) * t20285 + F::cast_from(0.55625000000000000001e-1_f64) * t20308 + F::cast_from(0.37083333333333333334e-1_f64) * t20312 - F::cast_from(0.30902777777777777778e-2_f64) * t20287 - F::cast_from(0.61805555555555555555e-2_f64) * t20315 + F::cast_from(0.18541666666666666667e-1_f64) * t20320 + F::cast_from(0.92708333333333333333e-2_f64) * t20290;
    t20567
}
