//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 890/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk890<F: Float>(t10133: F, t3192: F, t10103: F, t10106: F, t10108: F, t10111: F, t10115: F, t10118: F, t10120: F, t10126: F, t10128: F, t10131: F) -> (F, F) {
    let t10134 = t3192 * t10133;
    let t10136 = -F::new(0.27357942622625364862e-5) * t10103 + F::new(0.13223005600935593017e-4) * t10106 + F::new(0.1252584660908875509e-2) * t10108 + F::new(0.11742981196020707897e-5) * t10111 + F::new(0.18788769913633132635e-4) * t10115 + F::new(0.27357942622625364862e-5) * t10118 + F::new(0.94840867758434598187e-4) * t10120 + F::new(0.19798879235883268025e-5) * t10126 + F::new(0.14615314396567373048e-4) * t10128 - F::new(0.28183154870449698953e-3) * t10131 - F::new(0.18788769913633132635e-4) * t10134;
    (t10134, t10136)
}
