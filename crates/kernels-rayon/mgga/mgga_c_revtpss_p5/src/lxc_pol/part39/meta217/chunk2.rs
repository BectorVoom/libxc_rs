//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 868/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk868(t1469: f64, t999: f64, t4872: f64, t1042: f64, t1032: f64, t1647: f64, t1040: f64, t1025: f64, t1028: f64, t1041: f64, t1047: f64, t1665: f64, t1671: f64, t3124: f64, t3127: f64, t3194: f64, t3203: f64, t3211: f64, t3216: f64, t3224: f64, t4854: f64, t4858: f64, t4869: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4873 = t1469 * t999;
    let t4874 = t4872 * t4873;
    let t4875 = t1042 * t4874;
    let t4878 = t1647 * t1032;
    let t4879 = t4878 * t1040;
    let t4883 = -0.21437009059034868486e-3_f64 * t3224 * t1665 - 0.21437009059034868486e-3_f64 * t1025 * t4854 - 0.21437009059034868486e-3_f64 * t4858 * t1028 + 0.11433071498151929859e-2_f64 * t3211 * t1665 + 0.14291339372689912324e-3_f64 * t3194 - t3203 + 0.21437009059034868486e-3_f64 * t3124 * t1671 + 0.21437009059034868486e-3_f64 * t1041 * t4869 - 0.14291339372689912324e-3_f64 * t3127 * t4875 + 0.21437009059034868486e-3_f64 * t4879 * t1047 - 0.14291339372689912324e-3_f64 * t3216;
    (t4873, t4874, t4875, t4878, t4879, t4883)
}
