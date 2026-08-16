//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 587/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk587<F: Float>(t43: F, t1694: F, t886: F, t2868: F, t821: F, t1361: F, t234: F, t47: F, t5445: F, t822: F, t1699: F, t2876: F, t1702: F, t893: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t5450 = t886 * t1694;
    let t5455 = -F::cast_from(2.0_f64) * t821 - F::cast_from(6.0_f64) * t2868;
    let t5459 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5445 * t234 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1361 * t822 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5450 * t234 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t5455);
    let t5460 = t2876 * t1699;
    let t5465 = t893 * t1702;
    (t5455, t5459, t5460, t5465)
}
