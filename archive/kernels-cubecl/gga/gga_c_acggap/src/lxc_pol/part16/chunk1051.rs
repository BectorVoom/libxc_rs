//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1051/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1051<F: Float>(t467: F, t9476: F, t1298: F, t560: F, t10956: F, t1679: F, t469: F, t5506: F, t1427: F, t1953: F, t24753: F, t2541: F, t33388: F, t33403: F, t33409: F, t33412: F, t36575: F, t36602: F, t5439: F, t567: F, t7297: F, t8372: F, t9089: F, t9097: F) -> F {
    let t38559 = t9476 * t467;
    let t38563 = t1298 * t560;
    let t38571 = t1679 * t10956 * t560;
    let t38573 = t469 * t5506;
    let t38577 = -F::cast_from(6.0_f64) * t10956 * t5439 * t7297 - F::cast_from(6.0_f64) * t10956 * t7297 * t9089 + F::cast_from(12.0_f64) * t1427 * t36602 * t8372 + F::cast_from(3.0_f64) * t1953 * t38573 * t567 - F::cast_from(3.0_f64) * t24753 * t2541 * t7297 - F::cast_from(6.0_f64) * t2541 * t38563 * t7297 + F::cast_from(12.0_f64) * t38559 * t7297 * t9097 + t33388 - t33403 + t33409 + t33412 - t36575 - F::cast_from(2.0_f64) * t38571;
    t38577
}
