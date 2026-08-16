//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 791/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk791(t2584: f64, t2589: f64, t3461: f64, t3503: f64, t4236: f64, t4248: f64, t4252: f64, t4256: f64, t4258: f64, t4263: f64, t4267: f64) -> f64 {
    let t4323 = -0.1294625e1_f64 * t4248 + 0.258925e1_f64 * t4252 + t2584 - 0.60385e0_f64 * t3461 + 0.905775e0_f64 * t4236 + 0.82524375e-1_f64 * t4256 + 0.16504875e0_f64 * t4258 + t2589 - 0.33114e0_f64 * t3503 + 0.248355e0_f64 * t4263 + 0.248355e0_f64 * t4267;
    t4323
}
