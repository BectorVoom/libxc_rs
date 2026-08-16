//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1050/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1050(t36602: f64, t495: f64, t694: f64, t1717: f64, t301: f64, t9477: f64, t1662: f64, t560: f64, t10409: f64, t19418: f64, t2163: f64, t2355: f64, t33357: f64, t36610: f64, t38519: f64, t5399: f64, t5651: f64, t567: f64, t625: f64, t6596: f64, t6614: f64, t7278: f64, t7297: f64, t8372: f64, t9096: f64, t9097: f64, t9476: f64) -> f64 {
    let t38524 = t694 * t36602 * t495;
    let t38534 = t1717 * t301;
    let t38538 = t694 * t9477;
    let t38540 = t560 * t1662;
    let t38549 = -6.0_f64 * t10409 * t7297 * t9476 - t19418 * t567 * t625 + 2.0_f64 * t2163 * t567 * t6596 - t2163 * t567 * t6614 - 2.0_f64 * t2355 * t5399 * t567 - 6.0_f64 * t36610 * t38519 * t9096 + 6.0_f64 * t38534 * t7297 * t9097 + 4.0_f64 * t38540 * t9096 * t9097 + 6.0_f64 * t5651 * t7278 * t8372 - t33357 + 6.0_f64 * t38524 - 6.0_f64 * t38538;
    t38549
}
