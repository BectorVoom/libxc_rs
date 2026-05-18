//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 701/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk701<F: Float>(t1005: F, t1434: F, t2457: F, t2505: F, t2584: F, t2589: F, t3461: F, t3472: F, t3486: F, t3491: F, t3497: F, t3499: F, t3503: F, t3507: F, t3511: F) -> (F, F) {
    let t3565 = t1434 * t1005;
    let t3579 = -F::new(0.1294625e1) * t3486 + F::new(0.258925e1) * t3491 + t2584 - F::new(0.301925e0) * t2457 - F::new(0.301925e0) * t3461 + F::new(0.905775e0) * t3472 + F::new(0.82524375e-1) * t3497 + F::new(0.16504875e0) * t3499 + t2589 - F::new(0.16557e0) * t2505 - F::new(0.16557e0) * t3503 + F::new(0.248355e0) * t3507 + F::new(0.248355e0) * t3511;
    (t3565, t3579)
}
