//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2971/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2971(t11773: f64, t3278: f64, t11687: f64, t11774: f64, t12021: f64, t15584: f64, t15689: f64, t15691: f64, t15700: f64, t15701: f64, t15703: f64, t15809: f64, t16009: f64, t16013: f64, t1671: f64, t3095: f64, t3241: f64, t42235: f64, t42425: f64, t42699: f64, t42710: f64, t4786: f64, t4869: f64, t4875: f64, t53846: f64) -> (f64, f64) {
    let t54166 = t3278 * t11773;
    let t54176 = 0.57165357490759649295e-3_f64 * t42699 - 0.14481890564325777822e-1_f64 * t42425 * t4875 + 0.21437009059034868486e-3_f64 * t42235 * t1671 + 0.64311027177104605458e-3_f64 * t12021 * t4869 - 0.95275595817932748827e-4_f64 * t42710 - 0.42874018118069736972e-3_f64 * t15689 * t15691 * t11687 * t3095 - 0.42874018118069736972e-3_f64 * t11774 * t15584 * t15809 * t4786 - 0.17149607247227894789e-2_f64 * t54166 * t15703 - 0.17149607247227894789e-2_f64 * t15700 * t15701 * t53846 - t3241 * t16009 / 27.0_f64 - 7.0_f64 / 81.0_f64 * t3241 * t16013;
    (t54166, t54176)
}
