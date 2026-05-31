//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 738/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk738<F: Float>(t43: F, t50: F, t1361: F, t234: F, t47: F, t5445: F, t5450: F, t5455: F, t822: F, t1699: F, t2876: F, t1702: F, t893: F, t1369: F, t238: F, t52: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t5459 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5445 * t234 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1361 * t822 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5450 * t234 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t5455);
    let t5460 = t2876 * t1699;
    let t5465 = t893 * t1702;
    let t5468 = -t5455;
    let t5472 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5460 * t238 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1369 * t822 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5465 * t238 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t5468);
    (t5459, t5460, t5465, t5468, t5472)
}
