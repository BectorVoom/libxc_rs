//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 250/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk250(t1065: f64, t66: f64, t906: f64, t247: f64, t1003: f64, t1009: f64, t1011: f64, t1017: f64, t1021: f64, t1025: f64, t1028: f64, t1041: f64, t1047: f64, t1054: f64, t1060: f64, t1063: f64, t348: f64, t375: f64) -> (f64, f64, f64, f64) {
    let t1066 = t66 * t1065;
    let t1067 = t1066 * t906;
    let t1068 = t247 * t1067;
    let t1071 = -t1003 * t348 / 36.0_f64 + t1009 + t1011 * t1017 / 288.0_f64 + 0.21437009059034868486e-3_f64 * t1021 * t375 - 0.21437009059034868486e-3_f64 * t1025 * t1028 + 0.21437009059034868486e-3_f64 * t1041 * t1047 - 0.11433071498151929859e-2_f64 * t1054 * t375 + t1060 + 0.14291339372689912324e-3_f64 * t1063 * t1068;
    (t1066, t1067, t1068, t1071)
}
