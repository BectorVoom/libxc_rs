//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1747/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1747<F: Float>(t10416: F, t118: F, t13435: F, t1453: F, t2014: F, t2052: F, t2056: F, t2108: F, t2322: F, t2331: F, t25082: F, t25188: F, t26380: F, t26383: F, t26392: F, t26396: F, t26399: F, t26406: F, t26412: F, t26415: F, t26674: F, t26676: F, t26679: F, t26699: F, t3813: F, t508: F, t569: F, t651: F, t671: F, t7235: F, t7359: F, t7367: F, t7484: F, t7537: F) -> F {
    let t26702 = -F::new(2.0) * t2014 * t26380 + F::new(3.0) * t2014 * t26383 - F::new(2.0) * t10416 * t2056 - F::new(4.0) * t13435 * t2056 - F::new(4.0) * t2322 * t7367 - t2014 * t26392 - F::new(4.0) * t7359 * t2331 - F::new(4.0) * t651 * t26396 - F::new(4.0) * t26399 * t671 + F::new(2.0) * t7484 * t1453 + t25188 * t2108 - F::new(6.0) * t25082 * t26406 + F::new(2.0) * t7235 * t7537 + F::new(6.0) * t2014 * t26412 - F::new(2.0) * t651 * t26415 - t118 * t26674 - F::new(2.0) * t26676 * t508 + F::new(2.0) * t2014 * t26679 + t26699 * t569 - t2052 * t3813;
    t26702
}
