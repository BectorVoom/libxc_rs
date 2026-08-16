//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1294/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1294(t16451: f64, t16485: f64, t3734: f64, t571: f64, t1390: f64, t5356: f64, t12127: f64, t12133: f64, t12141: f64, t12466: f64, t1297: f64, t1307: f64, t15983: f64, t15985: f64, t15987: f64, t15988: f64, t16018: f64, t16165: f64, t16166: f64, t16167: f64, t16168: f64, t16171: f64, t16172: f64, t1799: f64, t193: f64, t3918: f64, t533: f64, t9853: f64, t9859: f64) -> f64 {
    let t16486 = t16451 + t16485;
    let t16490 = t3734 * t571;
    let t16497 = t5356 * t1390;
    let t16501 = t1390 * t16486 * t193 * t533 + 3.0_f64 * t12466 * t1799 * t3918 + 3.0_f64 * t1297 * t16018 * t193 + 6.0_f64 * t1307 * t16497 * t3918 + 6.0_f64 * t16490 * t1799 * t193 + t12127 + t12133 - t12141 + t15983 + t15985 - t15987 + t15988 + t16165 - t16166 + t16167 + t16168 - t16171 - t16172 + t9853 + t9859;
    t16501
}
