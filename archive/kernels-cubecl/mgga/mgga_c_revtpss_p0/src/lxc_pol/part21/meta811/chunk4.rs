//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2966/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2966<F: Float>(t16166: F, t3127: F, t3172: F, t16171: F, t11620: F, t11656: F, t15910: F, t16095: F, t16172: F, t3092: F, t3115: F, t3117: F, t3154: F, t42967: F, t4578: F, t4783: F, t4892: F, t4893: F, t4910: F, t53835: F, t54014: F, t54023: F, t54026: F, t54037: F, t54039: F) -> F {
    let t54042 = t3127 * t3172 * t16166;
    let t54047 = t3127 * t3172 * t16171;
    let t54049 = -F::cast_from(0.30488190661738479624e-2_f64) * t54014 - F::cast_from(0.45732285992607719436e-2_f64) * t42967 * t4783 + F::cast_from(0.42874018118069736972e-3_f64) * t4892 * t3117 * t4893 * t3154 * t11620 + F::cast_from(0.20579528696673473746e-1_f64) * t54023 * t15910 - F::cast_from(0.64311027177104605458e-3_f64) * t3115 * t3117 * t54026 * t4910 + F::cast_from(0.85748036236139473944e-3_f64) * t16095 * t3092 * t4578 * t53835 - t54037 - F::cast_from(0.57165357490759649295e-3_f64) * t54039 - F::cast_from(0.28582678745379824648e-3_f64) * t54042 + F::cast_from(0.3811023832717309953e-2_f64) * t11656 * t16172 - F::cast_from(0.47637797908966374413e-3_f64) * t54047;
    t54049
}
