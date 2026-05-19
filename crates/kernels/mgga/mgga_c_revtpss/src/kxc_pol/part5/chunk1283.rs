//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1283/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1283<F: Float>(t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F, t12296: F, t12297: F, t16706: F, t16915: F, t16916: F, t16917: F) -> (F, F) {
    let t20322 = F::cast_from(0.67094444444444444443e-1_f64) * t20283 - F::cast_from(0.20128333333333333333e0_f64) * t20285 - F::cast_from(0.10064166666666666667e0_f64) * t20287 + F::new(0.301925e0) * t20290 + F::cast_from(0.33547222222222222222e0_f64) * t20295 - F::new(0.12077e1) * t20300 - F::cast_from(0.40256666666666666666e0_f64) * t20304 + F::new(0.181155e1) * t20308 + F::new(0.12077e1) * t20312 - F::cast_from(0.20128333333333333333e0_f64) * t20315 + F::new(0.60385e0) * t20320;
    let t20337 = -t12296 + F::new(4.0) / F::new(27.0) * t12297 + F::new(8.0) / F::new(27.0) * t16706 + t16915 - t16916 - t16917 + F::new(2.0) / F::new(27.0) * t20283 + F::new(10.0) / F::new(27.0) * t20295 - F::new(4.0) / F::new(3.0) * t20300 - F::new(4.0) / F::new(9.0) * t20304 - F::new(2.0) / F::new(9.0) * t20285 + F::new(2.0) * t20308 + F::new(4.0) / F::new(3.0) * t20312 - t20287 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t20315 + F::new(2.0) / F::new(3.0) * t20320 + t20290 / F::new(3.0);
    (t20322, t20337)
}
