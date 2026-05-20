//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1388/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1388<F: Float>(t12297: F, t12678: F, t16706: F, t17319: F, t17320: F, t17321: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> F {
    let t21332 = -t12678 + F::cast_from(0.37037037037037037037e-2_f64) * t12297 + F::cast_from(0.74074074074074074074e-2_f64) * t16706 + t17319 - t17320 - t17321 + F::cast_from(0.18518518518518518518e-2_f64) * t20283 + F::cast_from(0.92592592592592592592e-2_f64) * t20295 - F::cast_from(0.33333333333333333333e-1_f64) * t20300 - F::cast_from(0.11111111111111111111e-1_f64) * t20304 - F::cast_from(0.55555555555555555557e-2_f64) * t20285 + F::cast_from(0.50000000000000000001e-1_f64) * t20308 + F::cast_from(0.33333333333333333334e-1_f64) * t20312 - F::cast_from(0.27777777777777777778e-2_f64) * t20287 - F::cast_from(0.55555555555555555555e-2_f64) * t20315 + F::cast_from(0.16666666666666666667e-1_f64) * t20320 + F::cast_from(0.83333333333333333333e-2_f64) * t20290;
    t21332
}
