//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1439/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1439<F: Float>(t12650: F, t2464: F, t2469: F, t2470: F, t36119: F, t36266: F, t36269: F, t36270: F, t36271: F, t36272: F, t36275: F, t36280: F, t36283: F, t36285: F, t36288: F, t36290: F, t36293: F, t36295: F, t38692: F, t3914: F, t7063: F, t972: F) -> F {
    let t38825 = F::new(4.0) * t12650 * t2469 * t972 - F::new(6.0) * t2470 * t3914 * t7063 - F::new(2.0) * t12650 * t2464 - t36119 + t36266 - t36269 + t36270 + t36271 - t36272 + t36275 + t36280 - t36283 + t36285 - t36288 - t36290 - t36293 - t36295 - t38692;
    t38825
}
