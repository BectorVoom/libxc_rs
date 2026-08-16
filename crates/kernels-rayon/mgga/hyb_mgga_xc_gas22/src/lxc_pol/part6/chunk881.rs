//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 881/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk881(t1046: f64, t2729: f64, t7453: f64, t1048: f64, t2712: f64, t2723: f64, t2728: f64, t567: f64, t222: f64, t2732: f64, t2702: f64, t2724: f64) -> (f64, f64, f64, f64, f64) {
    let t7456 = 0.48245938496077605201e2_f64 * t2729 * t7453 * t1046;
    let t7459 = 6.0_f64 * t2712 * t1048 * t2723;
    let t7460 = t567 * t2728;
    let t7463 = 0.85917975471764868594e0_f64 * t222 * t7460 * t2732;
    let t7466 = 0.53424999999999999999e-1_f64 * t222 * t2702 * t2724;
    (t7456, t7459, t7460, t7463, t7466)
}
