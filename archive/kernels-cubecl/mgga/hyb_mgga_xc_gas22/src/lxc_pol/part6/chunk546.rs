//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 546/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk546<F: Float>(t2454: F, t2502: F, t2457: F, t2468: F, t2486: F, t2491: F, t2497: F, t2499: F, t2505: F, t2509: F, t2513: F) -> (F, F, F) {
    let t2584 = F::cast_from(0.40256666666666666667e0_f64) * t2454;
    let t2589 = F::cast_from(0.137975e0_f64) * t2502;
    let t2593 = -F::cast_from(0.1294625e1_f64) * t2486 + F::cast_from(0.258925e1_f64) * t2491 + t2584 - F::cast_from(0.60385e0_f64) * t2457 + F::cast_from(0.905775e0_f64) * t2468 + F::cast_from(0.82524375e-1_f64) * t2497 + F::cast_from(0.16504875e0_f64) * t2499 + t2589 - F::cast_from(0.33114e0_f64) * t2505 + F::cast_from(0.248355e0_f64) * t2509 + F::cast_from(0.248355e0_f64) * t2513;
    (t2584, t2589, t2593)
}
