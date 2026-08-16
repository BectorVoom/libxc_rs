//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 746/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk746<F: Float>(t8326: F, t8366: F, t8405: F, t8439: F, t8481: F, t8519: F, t8555: F, t8593: F, t2962: F, t575: F, t1010: F, t1615: F) -> (F, F, F) {
    let t8596 = t8326 + t8366 + t8405 + t8439 + t8481 + t8519 + t8555 + t8593;
    let t8598 = t2962 * t575;
    let t8601 = t1010 * t1615;
    (t8596, t8598, t8601)
}
