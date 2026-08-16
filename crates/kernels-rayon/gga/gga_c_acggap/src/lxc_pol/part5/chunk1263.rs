//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1263/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1263(t6215: f64, t952: f64, t5950: f64, t997: f64, t3431: f64, t6237: f64, t3409: f64, t6241: f64, t3382: f64, t1017: f64, t1426: f64, t1459: f64, t1713: f64, t17139: f64, t176: f64, t17921: f64, t17926: f64, t17928: f64, t17930: f64, t17932: f64, t418: f64, t5735: f64, t8401: f64) -> f64 {
    let t23309 = t952 * t6215;
    let t23311 = t997 * t5950;
    let t23314 = t3431 * t6237;
    let t23316 = t3409 * t6241;
    let t23318 = t3382 * t6241;
    let t23320 = -0.17149607247227894789e-1_f64 * t17139 * t176 * t8401 * t5735 - 0.10289764348336736873e-1_f64 * t17921 - 0.68598428988911579156e-2_f64 * t17926 - 0.48018900292238105409e-1_f64 * t17928 - 0.32012600194825403606e-1_f64 * t17930 + 0.25724410870841842183e-1_f64 * t418 * t1426 * t1459 * t1713 * t1017 + 0.40015750243531754508e-2_f64 * t23309 - 0.80031500487063509015e-1_f64 * t23311 + 0.17149607247227894789e-2_f64 * t17932 - 0.80031500487063509015e-2_f64 * t23314 + 0.40015750243531754508e-2_f64 * t23316 - 0.85748036236139473944e-3_f64 * t23318;
    t23320
}
