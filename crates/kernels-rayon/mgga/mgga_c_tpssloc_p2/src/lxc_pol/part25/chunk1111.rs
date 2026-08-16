//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1111/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1111(t1307: f64, t6637: f64, t6888: f64, t81129: f64, t22747: f64, t22893: f64, t80681: f64, t154: f64, t9533: f64, t131: f64, t3748: f64, t2009: f64, t9537: f64) -> (f64, f64, f64, f64, f64) {
    let t81132 = t6888 * t6637 * t81129 * t1307;
    let t81140 = t80681 * t22893 * t22747;
    let t81142 = t9533 * t154;
    let t81144 = t81142 * t3748 * t131;
    let t81146 = t81144 * t9537 * t2009;
    (t81132, t81140, t81142, t81144, t81146)
}
