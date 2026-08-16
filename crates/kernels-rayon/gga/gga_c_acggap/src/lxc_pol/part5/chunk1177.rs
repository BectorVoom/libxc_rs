//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1177/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1177(t1131: f64, t1150: f64, t1180: f64, t1181: f64, t1532: f64, t16498: f64, t16500: f64, t16510: f64, t16524: f64, t1889: f64, t21331: f64, t21338: f64, t21340: f64, t21342: f64, t21348: f64, t335: f64, t372: f64, t4578: f64, t4593: f64, t5688: f64, t960: f64) -> f64 {
    let t21351 = t1150 * t4593 * t4578 / 4.0_f64 + t335 * t960 * t5688 * t372 / 24.0_f64 + t335 * t960 * t1889 * t1131 / 48.0_f64 - 7.0_f64 / 36.0_f64 * t21331 + 0.40015750243531754508e-2_f64 * t16498 + 0.16006300097412701803e-1_f64 * t16500 + 0.85748036236139473944e-3_f64 * t21338 - 0.34299214494455789578e-2_f64 * t21340 - 0.85748036236139473944e-3_f64 * t1180 * t1181 * t1532 * t21342 - 0.10289764348336736873e-1_f64 * t16510 - 0.32012600194825403606e-1_f64 * t21348 + 0.17149607247227894789e-2_f64 * t16524;
    t21351
}
