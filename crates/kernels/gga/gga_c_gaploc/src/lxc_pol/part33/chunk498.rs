//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 498/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk498<F: Float>(t1445: F, t2582: F, t2089: F, t935: F, t723: F, t1: F, t2536: F) -> (F, F, F, F, F) {
    let t2664 = t1445 * t2582;
    let t2667 = t2089 * t935;
    let t2668 = t2667 * t723;
    let t2669 = t1445 * t2668;
    let t2672 = t2536 * t1;
    (t2664, t2667, t2668, t2669, t2672)
}
