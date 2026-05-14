//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1226/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1226<F: Float>(t12297: F, t12367: F, t16706: F, t16820: F, t16821: F, t16822: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> (F,) {
    let t20567 = -t12367 + 0.41203703703703703703e-2 * t12297 + 0.82407407407407407408e-2 * t16706 + t16820 - t16821 - t16822 + 0.20601851851851851852e-2 * t20283 + 0.10300925925925925926e-1 * t20295 - 0.37083333333333333333e-1 * t20300 - 0.12361111111111111111e-1 * t20304 - 0.61805555555555555557e-2 * t20285 + 0.55625000000000000001e-1 * t20308 + 0.37083333333333333334e-1 * t20312 - 0.30902777777777777778e-2 * t20287 - 0.61805555555555555555e-2 * t20315 + 0.18541666666666666667e-1 * t20320 + 0.92708333333333333333e-2 * t20290;
    (t20567,)
}
