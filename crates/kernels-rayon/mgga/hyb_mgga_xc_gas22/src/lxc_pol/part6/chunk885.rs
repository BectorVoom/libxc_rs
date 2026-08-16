//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 885/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk885(t2635: f64, t2674: f64, t7515: f64, t1110: f64, t16: f64, t3021: f64, t492: f64, t1105: f64, t2699: f64, t2707: f64, t1052: f64, t2742: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7516 = t2635 * t2674 * t7515;
    let t7518 = 0.51947577317044391277e2_f64 * t1110 * t7516;
    let t7520 = t16 * t3021 * t492;
    let t7522 = 0.56968947174242584612e-3_f64 * t1105 * t7520;
    let t7523 = t2699 * t2707;
    let t7526 = t1052 * t2742;
    (t7516, t7518, t7520, t7522, t7523, t7526)
}
