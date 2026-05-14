//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 777/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk777<F: Float>(t2584: F, t2589: F, t3461: F, t3503: F, t4236: F, t4248: F, t4252: F, t4256: F, t4258: F, t4263: F, t4267: F) -> (F,) {
    let t4323 = -0.1294625e1 * t4248 + 0.258925e1 * t4252 + t2584 - 0.60385e0 * t3461 + 0.905775e0 * t4236 + 0.82524375e-1 * t4256 + 0.16504875e0 * t4258 + t2589 - 0.33114e0 * t3503 + 0.248355e0 * t4263 + 0.248355e0 * t4267;
    (t4323,)
}
