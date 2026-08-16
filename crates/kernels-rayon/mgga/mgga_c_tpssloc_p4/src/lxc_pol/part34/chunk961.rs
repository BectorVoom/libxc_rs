//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 961/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk961(t1653: f64, t6219: f64, t3578: f64, t1735: f64, t5971: f64, t11668: f64, t5979: f64, t1730: f64, t6164: f64, t2130: f64, t47: f64, t479: f64) -> (f64, f64, f64, f64, f64) {
    let t22153 = t6219 * t1653;
    let t22154 = t3578 * t22153;
    let t22157 = t1735 * t5971;
    let t22158 = t11668 * t22157;
    let t22161 = t1735 * t5979;
    let t22162 = t3578 * t22161;
    let t22169 = t1730 * t6164;
    let t22173 = 1.0_f64 / t47 / t2130;
    let t22174 = t479 * t22173;
    (t22154, t22158, t22162, t22169, t22174)
}
