//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2612/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2612(t11797: f64, t5005: f64, t1174: f64, t5045: f64, t698: f64, t3540: f64, t4966: f64, t11647: f64, t1744: f64, t11697: f64, t15469: f64, t3577: f64) -> (f64, f64, f64, f64, f64) {
    let t53267 = t5005 * t11797;
    let t53270 = t1174 * t698 * t5045;
    let t53272 = t4966 * t3540;
    let t53274 = t1744 * t11647;
    let t53287 = t3577 * t11697 * t15469;
    (t53267, t53270, t53272, t53274, t53287)
}
