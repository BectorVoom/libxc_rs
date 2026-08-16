//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1218/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1218(t10615: f64, t11486: f64, t3262: f64, t1563: f64, t3574: f64, t10997: f64, t10610: f64, t10918: f64, t11509: f64, t3579: f64, t36969: f64, t3261: f64, t498: f64, t97: f64) -> (f64, f64, f64, f64, f64) {
    let t40619 = 15.0_f64 / 8.0_f64 * t3262 * t10615 * t11486;
    let t40620 = t3574 * t1563;
    let t40623 = 135.0_f64 / 64.0_f64 * t3262 * t10997 * t40620;
    let t40626 = 3.0_f64 * t10610 * t10918 * t11509;
    let t40628 = 45.0_f64 / 64.0_f64 * t3579 * t36969;
    let t40630 = t97 * t3261 * t498;
    (t40619, t40623, t40626, t40628, t40630)
}
