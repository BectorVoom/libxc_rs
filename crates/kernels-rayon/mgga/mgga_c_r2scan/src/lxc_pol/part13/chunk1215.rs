//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1215/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1215(t11582: f64, t38248: f64, t38249: f64, t1065: f64, t2847: f64, t11002: f64, t3269: f64, t3274: f64, t5086: f64, t97: f64, t40296: f64, t792: f64) -> (f64, f64, f64, f64) {
    let t40587 = t38248 * t11582 * t38249;
    let t40589 = t1065 * t2847;
    let t40590 = t11002 * t40589;
    let t40592 = 5.0_f64 / 8.0_f64 * t3269 * t40590;
    let t40594 = t97 * t3274 * t5086;
    let t40595 = t40296 * t792;
    (t40587, t40592, t40594, t40595)
}
