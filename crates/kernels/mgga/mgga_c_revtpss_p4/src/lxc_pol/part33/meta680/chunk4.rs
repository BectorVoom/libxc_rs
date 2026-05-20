//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2218/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2218<F: Float>(t60673: F, t7565: F, t13272: F, t29411: F, t104279: F, t104282: F, t108769: F, t108792: F, t108864: F, t2123: F, t26792: F, t28133: F, t29412: F, t29562: F, t30686: F, t30689: F, t6960: F, t6963: F, t7566: F, t7706: F, t96824: F, t96827: F) -> F {
    let t111532 = t60673 * t7565;
    let t111537 = t13272 * t29411;
    let t111548 = F::new(5.0) / F::new(3.0) * t29412 * t28133 + F::new(2.0) / F::new(3.0) * t6963 * t30686 + F::new(5.0) / F::new(6.0) * t7566 * t108792 + t6963 * t30689 / F::new(3.0) - F::new(5.0) * t96824 * t29562 + F::new(5.0) / F::new(6.0) * t111532 * t6960 + t108769 * t2123 / F::new(3.0) + F::new(5.0) / F::new(3.0) * t111537 * t6960 - F::new(5.0) * t96827 * t29562 - F::new(5.0) * t26792 * t108864 + F::new(5.0) / F::new(3.0) * t104279 * t7706 + F::new(5.0) / F::new(3.0) * t104282 * t7706;
    t111548
}
