//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 912/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk912(t11714: f64, t478: f64, t10477: f64, t483: f64, t11713: f64, t1215: f64, t3507: f64, t3508: f64, t475: f64, t1214: f64, t248: f64, t3503: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11715 = 1.0_f64 / t11714;
    let t11716 = t11715 * t478;
    let t11717 = t483 * t10477;
    let t11718 = t11716 * t11717;
    let t11719 = t11713 * t11718;
    let t11720 = t3507 * t1215;
    let t11721 = t3508 * t475;
    let t11722 = t11720 * t11721;
    let t11724 = t248 * t1214 * t11722;
    let t11727 = t3503 * t11717;
    (t11715, t11717, t11719, t11720, t11721, t11724, t11727)
}
