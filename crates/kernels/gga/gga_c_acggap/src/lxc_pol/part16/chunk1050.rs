//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1050/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1050<F: Float>(t36602: F, t495: F, t694: F, t1717: F, t301: F, t9477: F, t1662: F, t560: F, t10409: F, t19418: F, t2163: F, t2355: F, t33357: F, t36610: F, t38519: F, t5399: F, t5651: F, t567: F, t625: F, t6596: F, t6614: F, t7278: F, t7297: F, t8372: F, t9096: F, t9097: F, t9476: F) -> F {
    let t38524 = t694 * t36602 * t495;
    let t38534 = t1717 * t301;
    let t38538 = t694 * t9477;
    let t38540 = t560 * t1662;
    let t38549 = -F::new(6.0) * t10409 * t7297 * t9476 - t19418 * t567 * t625 + F::new(2.0) * t2163 * t567 * t6596 - t2163 * t567 * t6614 - F::new(2.0) * t2355 * t5399 * t567 - F::new(6.0) * t36610 * t38519 * t9096 + F::new(6.0) * t38534 * t7297 * t9097 + F::new(4.0) * t38540 * t9096 * t9097 + F::new(6.0) * t5651 * t7278 * t8372 - t33357 + F::new(6.0) * t38524 - F::new(6.0) * t38538;
    t38549
}
