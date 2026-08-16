//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1996/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1996(t81074: f64, t22724: f64, t22727: f64, t22894: f64, t80670: f64, t3787: f64, t6955: f64, t154: f64, t9533: f64, t131: f64, t3748: f64, t2009: f64, t9537: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81075 = 0.16220877603642232915e0_f64 * t81074;
    let t81076 = t22724 * t22727;
    let t81080 = t80670 * t22894;
    let t81105 = t3787 * t6955;
    let t81142 = t9533 * t154;
    let t81144 = t81142 * t3748 * t131;
    let t81146 = t81144 * t9537 * t2009;
    (t81075, t81076, t81080, t81105, t81142, t81144, t81146)
}
