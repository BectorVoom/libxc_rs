//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1329/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1329<F: Float>(t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F, t12296: F, t12297: F, t16706: F, t16915: F, t16916: F, t16917: F) -> (F, F) {
    let t20322 = F::cast_from(0.67094444444444444443e-1_f64) * t20283 - F::cast_from(0.20128333333333333333e0_f64) * t20285 - F::cast_from(0.10064166666666666667e0_f64) * t20287 + F::cast_from(0.301925e0_f64) * t20290 + F::cast_from(0.33547222222222222222e0_f64) * t20295 - F::cast_from(0.12077e1_f64) * t20300 - F::cast_from(0.40256666666666666666e0_f64) * t20304 + F::cast_from(0.181155e1_f64) * t20308 + F::cast_from(0.12077e1_f64) * t20312 - F::cast_from(0.20128333333333333333e0_f64) * t20315 + F::cast_from(0.60385e0_f64) * t20320;
    let t20337 = -t12296 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12297 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t16706 + t16915 - t16916 - t16917 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t20283 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t20295 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t20300 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t20304 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t20285 + F::cast_from(2.0_f64) * t20308 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t20312 - t20287 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t20315 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t20320 + t20290 / F::cast_from(3.0_f64);
    (t20322, t20337)
}
