//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 717/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk717<F: Float>(t198: F, t207: F, t2392: F, t2393: F, t2394: F, t2400: F, t2402: F, t2403: F, t2404: F, t2408: F, t2411: F, t2416: F, t2430: F, t2569: F, t2614: F, t2617: F, t2832: F, t765: F, t775: F, t892: F) -> F {
    let t2836 = -t198 * t207 * t2408 * t2411 + t198 * t207 * t2832 * t892 + F::new(6.0) * t198 * t2393 * t2394 + F::new(3.0) * t198 * t2430 * t765 + F::new(6.0) * t2403 * t2404 * t775 + t2392 + t2400 + t2402 + t2416 - t2569 + t2614 + t2617;
    t2836
}
