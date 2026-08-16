//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1051/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1051(t467: f64, t9476: f64, t1298: f64, t560: f64, t10956: f64, t1679: f64, t469: f64, t5506: f64, t1427: f64, t1953: f64, t24753: f64, t2541: f64, t33388: f64, t33403: f64, t33409: f64, t33412: f64, t36575: f64, t36602: f64, t5439: f64, t567: f64, t7297: f64, t8372: f64, t9089: f64, t9097: f64) -> f64 {
    let t38559 = t9476 * t467;
    let t38563 = t1298 * t560;
    let t38571 = t1679 * t10956 * t560;
    let t38573 = t469 * t5506;
    let t38577 = -6.0_f64 * t10956 * t5439 * t7297 - 6.0_f64 * t10956 * t7297 * t9089 + 12.0_f64 * t1427 * t36602 * t8372 + 3.0_f64 * t1953 * t38573 * t567 - 3.0_f64 * t24753 * t2541 * t7297 - 6.0_f64 * t2541 * t38563 * t7297 + 12.0_f64 * t38559 * t7297 * t9097 + t33388 - t33403 + t33409 + t33412 - t36575 - 2.0_f64 * t38571;
    t38577
}
