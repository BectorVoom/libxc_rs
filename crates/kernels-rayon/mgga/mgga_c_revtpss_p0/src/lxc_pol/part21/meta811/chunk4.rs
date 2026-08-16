//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2966/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2966(t16166: f64, t3127: f64, t3172: f64, t16171: f64, t11620: f64, t11656: f64, t15910: f64, t16095: f64, t16172: f64, t3092: f64, t3115: f64, t3117: f64, t3154: f64, t42967: f64, t4578: f64, t4783: f64, t4892: f64, t4893: f64, t4910: f64, t53835: f64, t54014: f64, t54023: f64, t54026: f64, t54037: f64, t54039: f64) -> f64 {
    let t54042 = t3127 * t3172 * t16166;
    let t54047 = t3127 * t3172 * t16171;
    let t54049 = -0.30488190661738479624e-2_f64 * t54014 - 0.45732285992607719436e-2_f64 * t42967 * t4783 + 0.42874018118069736972e-3_f64 * t4892 * t3117 * t4893 * t3154 * t11620 + 0.20579528696673473746e-1_f64 * t54023 * t15910 - 0.64311027177104605458e-3_f64 * t3115 * t3117 * t54026 * t4910 + 0.85748036236139473944e-3_f64 * t16095 * t3092 * t4578 * t53835 - t54037 - 0.57165357490759649295e-3_f64 * t54039 - 0.28582678745379824648e-3_f64 * t54042 + 0.3811023832717309953e-2_f64 * t11656 * t16172 - 0.47637797908966374413e-3_f64 * t54047;
    t54049
}
