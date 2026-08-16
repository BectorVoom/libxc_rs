//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 430/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk430(t1025: f64, t1509: f64, t1032: f64, t1038: f64, t1501: f64, t141: f64, t1030: f64, t1037: f64, t1503: f64, t1043: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1510 = t1025 * t1509;
    let t1513 = t1032 * t1509;
    let t1515 = t1038 * t1501;
    let t1516 = t141 * t1515;
    let t1518 = 0.1898925e1_f64 * t1510 - t1030 + 0.29896666666666666667e0_f64 * t1503 + 0.3071625e0_f64 * t1513 - t1037 + 0.82156666666666666667e-1_f64 * t1516;
    let t1519 = t1518 * t1043;
    (t1510, t1513, t1515, t1516, t1518, t1519)
}
