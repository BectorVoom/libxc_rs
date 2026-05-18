//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 455/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk455<F: Float>(t2199: F, t2259: F, t2422: F, t2460: F, t880: F, t883: F, t337: F, t882: F) -> (F, F, F) {
    let t2462 = t2199 + t2259 + t2422 + t2460;
    let t2464 = t880 * t883;
    let t2468 = F::new(1.0) / t882 / t337;
    (t2462, t2464, t2468)
}
