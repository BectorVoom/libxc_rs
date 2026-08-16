//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3099/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3099(t16166: f64, t3127: f64, t3172: f64, t16171: f64, t42793: f64, t4899: f64, t4901: f64, t11710: f64, t16095: f64, t16097: f64, t16127: f64, t43131: f64) -> (f64, f64, f64, f64, f64) {
    let t54042 = t3127 * t3172 * t16166;
    let t54047 = t3127 * t3172 * t16171;
    let t54078 = t4899 * t42793 * t4901;
    let t54081 = t16095 * t11710 * t16097;
    let t54085 = t16095 * t43131 * t16127;
    (t54042, t54047, t54078, t54081, t54085)
}
