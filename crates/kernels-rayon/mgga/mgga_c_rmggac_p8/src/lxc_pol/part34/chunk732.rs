//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 732/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk732(t68401: f64, t68414: f64, t1550: f64, t699: f64, t7596: f64, t7617: f64, t903: f64, t2211: f64, t739: f64, t7840: f64, t3180: f64, t638: f64, t7184: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70797 = 0.36765206969775206063e-5_f64 * t68401;
    let t70799 = 0.51300288795035171252e-6_f64 * t68414;
    let t70806 = t1550 * t699 * t7596;
    let t70809 = t903 * t699 * t7617;
    let t70812 = t739 * t2211 * t7840;
    let t70818 = t638 * t7184 * t3180;
    (t70797, t70799, t70806, t70809, t70812, t70818)
}
