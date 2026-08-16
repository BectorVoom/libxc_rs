//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2042/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2042(t2371: f64, t5154: f64, t12134: f64, t12136: f64, t12138: f64, t5151: f64, t67: f64, t758: f64, t12142: f64, t12127: f64, t12133: f64, t12141: f64, t15980: f64, t15983: f64, t15985: f64, t15987: f64, t15988: f64, t9853: f64, t9859: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16164 = t5154 * t2371;
    let t16165 = 0.11696447245269292414e1_f64 * t16164;
    let t16166 = 16.0_f64 * t12134;
    let t16167 = 40.0_f64 * t12136;
    let t16168 = 0.23392894490538584828e1_f64 * t12138;
    let t16169 = t5151 * t67;
    let t16171 = 0.36622894612013090108e-3_f64 * t16169 * t758;
    let t16172 = 0.11696447245269292414e1_f64 * t12142;
    let t16173 = t15980 + t15983 + t15985 - t15987 + t12127 + t15988 + t12133 + t16165 - t16166 + t16167 + t9853 + t16168 - t16171 + t9859 - t12141 - t16172;
    (t16164, t16165, t16166, t16167, t16168, t16169, t16171, t16172, t16173)
}
