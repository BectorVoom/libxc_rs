//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 776/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk776(t548: f64, t6527: f64, t110: f64, t1567: f64, t252: f64, t146: f64) -> (f64, f64, f64, f64) {
    let t6528 = t6527 * t548;
    let t6533 = t110 * t1567;
    let t6534 = t6533 * t252;
    let t6535 = t146 * t6534;
    (t6528, t6533, t6534, t6535)
}
