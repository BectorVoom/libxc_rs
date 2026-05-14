//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 956/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk956<F: Float>(t4149: F, t874: F, t1305: F, t2293: F, t475: F, t588: F, t61: F) -> (F, F, F, F) {
    let t20065 = t4149 * t874;
    let t20073 = t874 * t1305;
    let t20117 = t2293 * t475;
    let t20157 = t61 * t588;
    (t20065, t20073, t20117, t20157)
}
