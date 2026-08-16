//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3004/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3004(t42415: f64, t4890: f64, t1062: f64, t42261: f64, t11913: f64, t15719: f64, t15850: f64, t15975: f64, t16049: f64, t3101: f64, t3299: f64, t3317: f64, t43029: f64, t43032: f64, t43035: f64, t43038: f64, t43121: f64, t4834: f64, t4896: f64, t4902: f64, t4912: f64) -> f64 {
    let t54885 = t42415 * t4890;
    let t54899 = t42261 * t1062;
    let t54904 = -0.64311027177104605458e-3_f64 * t43038 * t4912 - t43029 / 144.0_f64 + t43032 / 216.0_f64 + 0.43445671692977333464e-1_f64 * t3299 * t54885 * t4896 - 0.21722835846488666732e-1_f64 * t3317 * t54885 * t4902 + 0.68598428988911579154e-2_f64 * t43121 * t4912 - 0.42874018118069736972e-3_f64 * t43035 - 0.85748036236139473944e-3_f64 * t15850 * t3101 - 0.14291339372689912324e-2_f64 * t4834 * t11913 - 0.38586616306262763275e-2_f64 * t54899 * t15719 + 0.22866142996303859718e-2_f64 * t16049 * t15975;
    t54904
}
