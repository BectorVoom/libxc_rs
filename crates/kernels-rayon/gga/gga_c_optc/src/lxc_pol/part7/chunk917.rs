//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 917/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk917(t1035: f64, t3016: f64, t375: f64, t3019: f64, t388: f64, t8561: f64, t3053: f64, t3058: f64, t4219: f64, t1102: f64, t1084: f64, t3057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8685 = 1.0_f64 / t3016 / t1035;
    let t8686 = t375 * t8685;
    let t8688 = 1.0_f64 / t3019 / t388;
    let t8689 = t8561 * t8688;
    let t8691 = 0.51725014705706168417e3_f64 * t8686 * t8689;
    let t8693 = t3058 * t3053 * t4219;
    let t8695 = 0.51947267698127589897e2_f64 * t1102 * t8693;
    let t8697 = 1.0_f64 / t3057 / t1084;
    (t8685, t8686, t8688, t8689, t8691, t8693, t8695, t8697)
}
