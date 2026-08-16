//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 849/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk849(t2243: f64, t6137: f64, t2238: f64, t348: f64, t338: f64) -> (f64, f64, f64) {
    let t6139 = 0.48245938496077605201e2_f64 * t6137 * t2243;
    let t6141 = 1.0_f64 / t2238 / t348;
    let t6142 = t338 * t6141;
    (t6139, t6141, t6142)
}
