//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 640/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk640(t1962: f64, t1967: f64, t2730: f64, t2772: f64, t3517: f64, t3529: f64, t3533: f64, t3537: f64, t3539: f64, t3544: f64, t3548: f64) -> f64 {
    let t3604 = -0.1294625e1_f64 * t3529 + 0.258925e1_f64 * t3533 + t1962 - 0.60385e0_f64 * t2730 + 0.905775e0_f64 * t3517 + 0.82524375e-1_f64 * t3537 + 0.16504875e0_f64 * t3539 + t1967 - 0.33114e0_f64 * t2772 + 0.248355e0_f64 * t3544 + 0.248355e0_f64 * t3548;
    t3604
}
