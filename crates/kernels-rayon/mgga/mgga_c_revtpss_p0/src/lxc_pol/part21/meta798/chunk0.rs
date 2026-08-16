//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2890/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2890(t15475: f64, t2869: f64, t11385: f64, t1609: f64, t11387: f64, t2918: f64, t934: f64, t41578: f64, t4636: f64, t11528: f64, t15380: f64, t11294: f64, t15390: f64) -> (f64, f64, f64, f64, f64) {
    let t52481 = 3.0_f64 * t2869 * t15475;
    let t52482 = t11385 * t1609;
    let t52486 = 0.1551780387578202009e4_f64 * t52482 * t11387 * t2918 * t934;
    let t52488 = 0.48245938496077605201e2_f64 * t41578 * t4636;
    let t52490 = 12.0_f64 * t11528 * t15380;
    let t52492 = 0.96491876992155210402e2_f64 * t11294 * t15390;
    (t52481, t52486, t52488, t52490, t52492)
}
