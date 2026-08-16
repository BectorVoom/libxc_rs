//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2445/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2445(t10189: f64, t3008: f64, t4509: f64, t13797: f64, t984: f64, t10216: f64, t343: f64, t3152: f64, t698: f64, t973: f64, t10870: f64, t3117: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43057 = t10189 * t3008;
    let t43065 = t4509 * t3008;
    let t43069 = t13797 * t984;
    let t43070 = t343 * t10216;
    let t43110 = t973 * t698 * t3152;
    let t43114 = t3117 * t10870;
    (t43057, t43065, t43069, t43070, t43110, t43114)
}
