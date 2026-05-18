//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 517/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk517<F: Float>(t2455: F, t2457: F, t2468: F, t361: F, t944: F, t948: F) -> (F, F, F) {
    let t2470 = t2455 - F::new(0.35616666666666666666e-1) * t2457 + F::new(0.53425e-1) * t2468;
    let t2472 = F::new(0.621814e-1) * t2470 * t361;
    let t2473 = t944 * t948;
    (t2470, t2472, t2473)
}
