//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 868/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk868<F: Float>(t1042: F, t23858: F, t1592: F, t19649: F, t1015: F, t22671: F, t1012: F, t1011: F, t1041: F, t1063: F, t11246: F, t11256: F, t11630: F, t11927: F, t15707: F, t15823: F, t15932: F, t1671: F, t19659: F, t19697: F, t23630: F, t23635: F, t23643: F, t23823: F, t23830: F, t23834: F, t23839: F, t23844: F, t23848: F, t23852: F, t3127: F, t4837: F, t4879: F, t6263: F, t6302: F, t6308: F, t6312: F) -> (F,) {
    let t23859 = t1042 * t23858;
    let t23862 = t19649 * t1592;
    let t23863 = t1042 * t23862;
    let t23868 = t1015 * t22671;
    let t23869 = t1012 * t23868;
    let t23872 = 0.85748036236139473944e-3 * t1063 * t23630 + 0.85748036236139473944e-3 * t3127 * t23635 - 0.64311027177104605458e-3 * t15932 * t6312 + 0.21437009059034868486e-3 * t11256 * t23643 + 0.64311027177104605458e-3 * t4879 * t6302 + 0.21437009059034868486e-3 * t1041 * t23823 + 0.12862205435420921092e-2 * t15823 * t6308 + 0.42874018118069736972e-3 * t19659 + 0.12862205435420921092e-2 * t11630 * t23830 - 0.12862205435420921092e-2 * t11246 * t23834 + 0.12862205435420921092e-2 * t11927 * t23839 + 0.71456696863449561621e-3 * t1063 * t23844 - 0.7145669686344956162e-3 * t3127 * t23848 - 0.85748036236139473944e-3 * t1063 * t23852 + 0.64311027177104605458e-3 * t19697 * t1671 - 0.42874018118069736972e-3 * t3127 * t23859 + 0.85748036236139473944e-3 * t4837 * t23863 - 0.85748036236139473944e-3 * t15707 * t6263 + t1011 * t23869 / 288.0;
    (t23872,)
}
