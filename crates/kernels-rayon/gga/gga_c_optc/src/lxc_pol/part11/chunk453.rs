//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 453/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk453(t2620: f64, t331: f64, t287: f64, t530: f64, t321: f64, t320: f64, t327: f64, t301: f64, t2665: f64, t305: f64, t140: f64, t2661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2729 = 0.16793568152788065763e-2_f64 * t331 * t2620;
    let t2742 = t530 * t287;
    let t2743 = t321 * t2742;
    let t2745 = 0.19318136643975017455e-1_f64 * t320 * t2743;
    let t2746 = t327 * t327;
    let t2747 = 1.0_f64 / t2746;
    let t2748 = t2747 * t301;
    let t2749 = t305 * t2665;
    let t2750 = t2749 * t140;
    let t2751 = t2748 * t2750;
    let t2758 = t2661 * t2750;
    (t2729, t2742, t2745, t2746, t2747, t2748, t2751, t2758)
}
