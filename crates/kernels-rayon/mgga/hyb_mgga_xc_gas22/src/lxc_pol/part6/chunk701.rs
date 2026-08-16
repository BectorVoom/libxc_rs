//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 701/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk701(t1005: f64, t1434: f64, t2457: f64, t2505: f64, t2584: f64, t2589: f64, t3461: f64, t3472: f64, t3486: f64, t3491: f64, t3497: f64, t3499: f64, t3503: f64, t3507: f64, t3511: f64) -> (f64, f64) {
    let t3565 = t1434 * t1005;
    let t3579 = -0.1294625e1_f64 * t3486 + 0.258925e1_f64 * t3491 + t2584 - 0.301925e0_f64 * t2457 - 0.301925e0_f64 * t3461 + 0.905775e0_f64 * t3472 + 0.82524375e-1_f64 * t3497 + 0.16504875e0_f64 * t3499 + t2589 - 0.16557e0_f64 * t2505 - 0.16557e0_f64 * t3503 + 0.248355e0_f64 * t3507 + 0.248355e0_f64 * t3511;
    (t3565, t3579)
}
