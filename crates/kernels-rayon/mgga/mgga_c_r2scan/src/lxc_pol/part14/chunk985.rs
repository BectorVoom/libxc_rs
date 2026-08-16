//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 985/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk985(t2813: f64, t3446: f64, t3453: f64, t3308: f64, t3692: f64, t3429: f64, t2816: f64, t1102: f64, t3314: f64, t3582: f64, t792: f64, t6967: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11601 = t3446 * t3453 * t2813;
    let t11603 = t3308 * t3692;
    let t11604 = t3429 * t11603;
    let t11607 = t3446 * t3453 * t2816;
    let t11616 = t1102 * t3314 * t3692;
    let t11621 = t3582 * t792;
    let t11625 = t6967 * t795;
    (t11601, t11603, t11604, t11607, t11616, t11621, t11625)
}
