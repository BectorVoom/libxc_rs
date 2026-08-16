//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 546/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk546(t2454: f64, t2502: f64, t2457: f64, t2468: f64, t2486: f64, t2491: f64, t2497: f64, t2499: f64, t2505: f64, t2509: f64, t2513: f64) -> (f64, f64, f64) {
    let t2584 = 0.40256666666666666667e0_f64 * t2454;
    let t2589 = 0.137975e0_f64 * t2502;
    let t2593 = -0.1294625e1_f64 * t2486 + 0.258925e1_f64 * t2491 + t2584 - 0.60385e0_f64 * t2457 + 0.905775e0_f64 * t2468 + 0.82524375e-1_f64 * t2497 + 0.16504875e0_f64 * t2499 + t2589 - 0.33114e0_f64 * t2505 + 0.248355e0_f64 * t2509 + 0.248355e0_f64 * t2513;
    (t2584, t2589, t2593)
}
