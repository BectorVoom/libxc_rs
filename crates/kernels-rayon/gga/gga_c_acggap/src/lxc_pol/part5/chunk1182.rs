//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1182/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1182(t5529: f64, t997: f64, t12854: f64, t1817: f64, t1165: f64, t1173: f64, t13121: f64, t13128: f64, t13133: f64, t13135: f64, t13137: f64, t13146: f64, t16612: f64, t16625: f64, t1889: f64, t407: f64, t5688: f64, t930: f64) -> f64 {
    let t21484 = t997 * t5529;
    let t21486 = t12854 * t1817;
    let t21489 = -0.40015750243531754508e-2_f64 * t16612 + 0.17149607247227894789e-2_f64 * t1173 * t1165 * t5688 * t407 + 0.85748036236139473944e-3_f64 * t1173 * t1165 * t1889 * t930 - 0.17149607247227894789e-2_f64 * t16625 - 0.34299214494455789578e-2_f64 * t13121 - 0.10289764348336736874e-1_f64 * t13128 - 0.17149607247227894789e-2_f64 * t13133 + 0.51448821741683684366e-2_f64 * t13135 - 0.85748036236139473945e-2_f64 * t13137 - 0.24009450146119052706e-1_f64 * t21484 - 0.40015750243531754508e-2_f64 * t21486 - 0.25724410870841842183e-2_f64 * t13146;
    t21489
}
