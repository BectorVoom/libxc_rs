//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 951/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk951(t10469: f64, t466: f64, t10471: f64, t1208: f64, t478: f64, t10477: f64, t483: f64, t3508: f64, t475: f64, t3503: f64, t11708: f64, t3514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11712 = t466 * t10469;
    let t11713 = t11712 * t10471;
    let t11714 = t1208 * t1208;
    let t11715 = 1.0_f64 / t11714;
    let t11716 = t11715 * t478;
    let t11717 = t483 * t10477;
    let t11718 = t11716 * t11717;
    let t11719 = t11713 * t11718;
    let t11721 = t3508 * t475;
    let t11727 = t3503 * t11717;
    let t11728 = t11713 * t11727;
    let t11734 = t11708 * t3514;
    (t11712, t11713, t11715, t11717, t11719, t11721, t11728, t11734)
}
