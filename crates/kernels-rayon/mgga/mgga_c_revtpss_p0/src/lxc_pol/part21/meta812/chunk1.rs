//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2970/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2970(t11921: f64, t15837: f64, t247: f64, t4837: f64, t11267: f64, t4878: f64, t11263: f64, t4879: f64, t1047: f64, t1068: f64, t11714: f64, t11875: f64, t15606: f64, t3116: f64, t3117: f64, t42830: f64, t4831: f64, t4893: f64, t54112: f64, t54118: f64, t54123: f64, t54127: f64, t54130: f64, t54137: f64) -> f64 {
    let t54142 = t4837 * t247 * t11921 * t15837;
    let t54144 = t4878 * t11267;
    let t54147 = t4879 * t11263;
    let t54148 = 0.14291339372689912324e-3_f64 * t54147;
    let t54149 = 0.12862205435420921092e-2_f64 * t4837 * t247 * t3116 * t54112 + 5.0_f64 / 3888.0_f64 * t54118 + t54123 - t54127 + 0.12862205435420921092e-2_f64 * t42830 * t15606 + 0.64311027177104605458e-3_f64 * t11875 * t3117 * t4893 * t54130 - 0.45732285992607719436e-2_f64 * t11714 * t4831 + 0.42874018118069736972e-3_f64 * t54137 * t1068 + 0.85748036236139473944e-3_f64 * t54142 + 0.21722835846488666732e-1_f64 * t54144 * t1047 - t54148;
    t54149
}
