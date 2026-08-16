//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2988/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2988(t12021: f64, t4820: f64, t11998: f64, t15822: f64, t1042: f64, t11151: f64, t11774: f64, t15584: f64, t15586: f64, t15599: f64, t15907: f64, t15950: f64, t16081: f64, t16082: f64, t16170: f64, t1671: f64, t3097: f64, t3117: f64, t3127: f64, t3164: f64, t42155: f64, t42690: f64, t42970: f64, t4786: f64, t4873: f64, t54469: f64, t54471: f64, t54474: f64, t54479: f64) -> f64 {
    let t54490 = t12021 * t4820;
    let t54492 = t15822 * t11998;
    let t54495 = -0.85748036236139473944e-3_f64 * t42155 * t15586 - 0.85748036236139473944e-3_f64 * t11774 * t15584 * t15950 * t4786 - 0.42874018118069736972e-3_f64 * t11774 * t15584 * t4873 * t15599 + 0.17149607247227894789e-2_f64 * t54469 - 0.45732285992607719436e-2_f64 * t54471 * t3097 - 0.64311027177104605458e-3_f64 * t42690 * t3117 * t15907 * t54474 + 0.38586616306262763275e-2_f64 * t16081 * t3117 * t54479 * t16082 + 0.14291339372689912324e-2_f64 * t3127 * t1042 * t16170 * t11151 - 0.34299214494455789577e-2_f64 * t42970 * t1671 + 0.42874018118069736972e-3_f64 * t54490 + 0.34299214494455789577e-2_f64 * t54492 * t3164;
    t54495
}
