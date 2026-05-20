//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 990/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk990<F: Float>(t10406: F, t77: F, t10317: F, t10318: F, t10321: F, t10328: F, t10331: F, t10336: F, t10381: F, t2252: F, t2260: F, t2263: F, t2292: F, t2312: F, t608: F, t628: F, t641: F, t71: F, t85: F) -> (F, F) {
    let t10407 = t77 * t10406;
    let t10410 = -t10317 * t10318 / F::new(4.0) - t10321 * t85 / F::new(4.0) - t2252 * t641 / F::new(4.0) - t10328 * t85 / F::new(12.0) - t10331 * t85 / F::new(4.0) - t2260 * t641 / F::new(4.0) - t10336 * t85 / F::new(4.0) - t2263 * t641 / F::new(2.0) - t608 * t2312 / F::new(4.0) + t10381 * t85 / F::new(24.0) + t2292 * t641 / F::new(8.0) + t628 * t2312 / F::new(8.0) + t71 * t10407 / F::new(24.0);
    (t10407, t10410)
}
