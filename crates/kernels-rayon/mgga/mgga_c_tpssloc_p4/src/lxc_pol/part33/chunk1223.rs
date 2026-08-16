//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1223/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1223(t2010: f64, t81071: f64, t6973: f64, t80742: f64, t154: f64, t9533: f64, t131: f64, t3748: f64, t2009: f64, t9537: f64, t2690: f64, t22691: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t81072 = t81071 * t2010;
    let t81073 = 0.27720185200590482541e0_f64 * t81072;
    let t81074 = t80742 * t6973;
    let t81075 = 0.16220877603642232915e0_f64 * t81074;
    let t81142 = t9533 * t154;
    let t81144 = t81142 * t3748 * t131;
    let t81146 = t81144 * t9537 * t2009;
    let t81147 = 0.13707783890401886971e-2_f64 * t81146;
    let t81151 = t2690 * t154;
    let t81152 = t81151 * t3748;
    let t81153 = t81152 * t22691;
    (t81073, t81075, t81142, t81144, t81147, t81151, t81152, t81153)
}
