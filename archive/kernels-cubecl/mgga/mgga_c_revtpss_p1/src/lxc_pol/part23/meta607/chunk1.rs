//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2270/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2270<F: Float>(t12367: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F, t448: F) -> (F, F) {
    let t24252 = -t12367 + F::cast_from(0.12361111111111111111e-1_f64) * t16706 + F::cast_from(0.61805555555555555556e-2_f64) * t20283 - F::cast_from(0.18541666666666666667e-1_f64) * t20285 - F::cast_from(0.92708333333333333334e-2_f64) * t20287 + F::cast_from(0.10300925925925925926e-1_f64) * t24230 - F::cast_from(0.37083333333333333333e-1_f64) * t24234 - F::cast_from(0.18541666666666666666e-1_f64) * t24238 + F::cast_from(0.55625000000000000001e-1_f64) * t24242 + F::cast_from(0.55625000000000000001e-1_f64) * t24246 + F::cast_from(0.92708333333333333333e-2_f64) * t24250;
    let t24253 = t24252 * t448;
    (t24252, t24253)
}
