//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1178/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1178(t259: f64, t461: f64, t467: f64, t495: f64, t2723: f64, t1047: f64, t2712: f64, t2713: f64, t7449: f64, t2674: f64, t1110: f64, t2635: f64, t2639: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21832 = 24.0_f64 * t461 * t467 * t259 * t495;
    let t21833 = t2723 * t2723;
    let t21836 = 6.0_f64 * t2712 * t21833 * t1047;
    let t21837 = t2713 * t2713;
    let t21840 = 24.0_f64 * t7449 * t21837 * t1047;
    let t21841 = t2674 * t2674;
    let t21845 = 0.51947577317044391277e2_f64 * t1110 * t2635 * t21841 * t2639;
    (t21832, t21833, t21836, t21837, t21840, t21841, t21845)
}
