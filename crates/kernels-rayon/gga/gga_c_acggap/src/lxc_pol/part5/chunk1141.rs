//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1141/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1141(t1867: f64, t3570: f64, t5946: f64, t997: f64, t14056: f64, t6328: f64, t12587: f64, t12599: f64, t12601: f64, t12603: f64, t12608: f64, t15486: f64, t15497: f64, t15501: f64, t15508: f64) -> f64 {
    let t20453 = t3570 * t1867;
    let t20455 = t997 * t5946;
    let t20459 = t14056 * t6328;
    let t20467 = -0.20579528696673473746e-1_f64 * t15486 + 35.0_f64 / 108.0_f64 * t20453 + 0.80031500487063509015e-1_f64 * t20455 + 0.10289764348336736873e-1_f64 * t15497 - 0.85748036236139473944e-3_f64 * t12587 + 0.13719685797782315831e-1_f64 * t20459 + 0.96037800584476210817e-1_f64 * t15501 - 0.90702367218671976886e-1_f64 * t12599 + 0.90702367218671976886e-1_f64 * t12601 - 0.68026775414003982664e-1_f64 * t12603 + 0.25724410870841842183e-2_f64 * t15508 - 0.42874018118069736972e-2_f64 * t12608;
    t20467
}
