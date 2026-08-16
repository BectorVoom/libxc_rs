//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1021/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1021(t1796: f64, t3648: f64, t603: f64, t509: f64, t6636: f64, t6642: f64, t1772: f64, t1998: f64, t1994: f64, t6814: f64, t2041: f64, t35: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22697 = 0.67471169937307261776e-1_f64 * t1796 * t3648 * t603;
    let t22703 = 0.38024868119570572865e2_f64 * t1796 * t509 * t6636;
    let t22708 = 0.21687161765563048428e-1_f64 * t1796 * t509 * t6642;
    let t22711 = 0.43374323531126096856e-1_f64 * t1796 * t1772 * t1998;
    let t22716 = 0.1284251895870376528e1_f64 * t1796 * t1772 * t1994;
    let t22719 = 0.38527556876111295841e1_f64 * t1796 * t509 * t6814;
    let t22723 = t35 * t2041 * t88;
    (t22697, t22703, t22708, t22711, t22716, t22719, t22723)
}
