//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 464/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk464<F: Float>(t2293: F, t569: F, t568: F, t600: F, t524: F, t894: F) -> (F, F, F, F, F) {
    let t2427 = t569 * t2293;
    let t2428 = t568 * t2427;
    let t2433 = t600 * t2293;
    let t2434 = t568 * t2433;
    let t2437 = t524 * t894;
    (t2427, t2428, t2433, t2434, t2437)
}
