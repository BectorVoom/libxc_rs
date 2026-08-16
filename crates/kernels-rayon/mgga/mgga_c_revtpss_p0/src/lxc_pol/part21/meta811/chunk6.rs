//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2968/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2968(t16095: f64, t16127: f64, t43131: f64, t16088: f64, t3046: f64, t380: f64, t16139: f64, t3127: f64, t3172: f64, t1042: f64, t11933: f64, t15922: f64, t16089: f64, t16098: f64, t16152: f64, t2853: f64, t3092: f64, t3181: f64, t42637: f64, t42656: f64, t42658: f64, t42660: f64, t42662: f64, t4772: f64, t906: f64) -> (f64, f64) {
    let t54085 = t16095 * t43131 * t16127;
    let t54089 = t3046 * t380 * t16088;
    let t54099 = t3127 * t3172 * t16139;
    let t54110 = -0.95275595817932748827e-3_f64 * t54085 + 0.57165357490759649295e-3_f64 * t42637 + 0.17149607247227894789e-2_f64 * t54089 * t16098 + 0.17149607247227894789e-2_f64 * t16089 * t3092 * t16152 * t906 + 0.68598428988911579154e-2_f64 * t11933 * t15922 - 0.57165357490759649295e-3_f64 * t54099 + 0.15244095330869239812e-2_f64 * t42656 - 0.45732285992607719436e-2_f64 * t42658 - 0.45732285992607719436e-2_f64 * t42660 + 0.22866142996303859718e-2_f64 * t42662 - 0.7145669686344956162e-3_f64 * t3127 * t1042 * t3181 * t4772 * t2853;
    (t54089, t54110)
}
