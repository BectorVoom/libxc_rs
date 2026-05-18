//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 472/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk472<F: Float>(t2334: F, t475: F, t1445: F, t2345: F, t188: F, t2304: F, t1645: F, t494: F) -> (F, F, F, F, F) {
    let t2378 = t2334 * t475;
    let t2379 = t1445 * t2378;
    let t2382 = t1445 * t2345;
    let t2385 = t188 * t2304;
    let t2386 = t1645 * t494;
    (t2378, t2379, t2382, t2385, t2386)
}
