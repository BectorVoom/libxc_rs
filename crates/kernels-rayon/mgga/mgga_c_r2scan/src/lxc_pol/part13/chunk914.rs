//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 914/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk914(t1569: f64, t2530: f64, t795: f64, t910: f64, t2441: f64, t352: f64, t104: f64, t114: f64, t97: f64) -> (f64, f64, f64, f64, f64) {
    let t9507 = t1569 * t2530;
    let t9577 = t910 * t795;
    let t9760 = t352 * t2441;
    let t10609 = t104 * t114;
    let t10610 = t97 * t10609;
    (t9507, t9577, t9760, t10609, t10610)
}
