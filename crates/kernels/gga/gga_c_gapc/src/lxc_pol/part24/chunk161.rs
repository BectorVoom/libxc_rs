//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 161/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk161<F: Float>(t457: F, t73: F, t115: F, t154: F, t155: F, t169: F, t405: F, t528: F, t532: F, t536: F, t561: F, t563: F) -> (F, F) {
    let t564 = t73 * t457;
    let t567 = F::new(0.53062222222222222221e-1) * t154 * t528 * t115 + F::new(0.79593333333333333331e-1) * t154 * t532 * t115 - F::new(0.79593333333333333331e-1) * t154 * t155 * t536 - t561 * t73 + t563 * t564 - t169 * t405;
    (t564, t567)
}
