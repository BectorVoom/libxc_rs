//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2925/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2925(t17934: f64, t2944: f64, t10623: f64, t5804: f64, t59981: f64, t60006: f64, t60008: f64, t60010: f64, t60016: f64, t60021: f64, t60023: f64, t60025: f64, t60027: f64, t60029: f64, t60033: f64, t60035: f64) -> (f64, f64, f64) {
    let t60906 = 0.11696447245269292414e1_f64 * t17934 * t2944;
    let t60908 = 0.11696447245269292414e1_f64 * t10623 * t5804;
    let t60909 = t60906 + t60908 + t59981 - t60006 + t60008 - t60010 - t60016 + t60021 + t60023 - t60025 - t60027 - t60029 + t60033 - t60035;
    (t60906, t60908, t60909)
}
