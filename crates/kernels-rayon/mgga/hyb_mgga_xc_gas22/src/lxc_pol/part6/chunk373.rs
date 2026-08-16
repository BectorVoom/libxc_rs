//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 373/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk373(t1363: f64, t284: f64, t1331: f64, t1337: f64, t1340: f64, t1344: f64, t842: f64, t845: f64) -> (f64, f64) {
    let t1364 = t1363 * t284;
    let t1370 = 0.258925e1_f64 * t1337 - t842 + 0.905775e0_f64 * t1331 + 0.16504875e0_f64 * t1340 - t845 + 0.248355e0_f64 * t1344;
    (t1364, t1370)
}
