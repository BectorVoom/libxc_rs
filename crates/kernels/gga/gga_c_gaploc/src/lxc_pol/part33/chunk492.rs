//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 492/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk492<F: Float>(t2513: F, t2515: F, t2520: F, t2522: F, t471: F, t64: F, t931: F) -> (F, F) {
    let t2524 = -F::new(21.0) / F::new(256.0) * t2513 + F::new(21.0) / F::new(8192.0) * t2515 - F::new(7.0) / F::new(8192.0) * t2520 + F::new(7.0) / F::new(256.0) * t2522;
    let t2530 = t2524 * t471 - F::new(4.0) / F::new(3.0) * t931 * t64 - F::new(7.0) / F::new(256.0) * t2513 + F::new(7.0) / F::new(768.0) * t2522;
    (t2524, t2530)
}
