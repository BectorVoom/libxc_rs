//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 538/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk538<F: Float>(t1218: F, t1253: F, t301: F, t317: F, t5207: F, t5305: F, t5310: F, t5380: F, t5394: F, t5422: F, t5424: F, t332: F) -> (F, F) {
    let t5429 = -F::new(2.0) * t1218 * t1253 - t301 * t5422 - t317 * t5207 - t317 * t5305 + F::new(4.0) * t5310 - F::new(4.0) * t5380 - F::new(2.0) * t5394 + F::new(2.0) * t5424;
    let t5430 = t5429 * t332;
    (t5429, t5430)
}
