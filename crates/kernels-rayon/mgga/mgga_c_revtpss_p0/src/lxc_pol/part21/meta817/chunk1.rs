//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3006/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3006(t1043: f64, t43116: f64, t3075: f64, t4900: f64, t1063: f64, t11986: f64, t247: f64, t4583: f64, t11859: f64, t11875: f64, t15609: f64, t15703: f64, t15780: f64, t3117: f64, t3120: f64, t4893: f64, t4899: f64, t53923: f64, t54907: f64, t54909: f64, t54914: f64, t54916: f64, t54919: f64, t54925: f64) -> (f64, f64, f64) {
    let t54931 = t43116 * t1043;
    let t54936 = t4900 * t3075;
    let t54943 = t1063 * t247 * t11986 * t4583;
    let t54945 = -0.28582678745379824648e-3_f64 * t54907 - 0.64311027177104605458e-3_f64 * t4899 * t3117 * t54909 * t4900 - 0.85748036236139473944e-3_f64 * t54914 + 0.68598428988911579154e-2_f64 * t54916 * t3120 - 0.85748036236139473944e-3_f64 * t54919 + 0.91464571985215438873e-2_f64 * t53923 * t15703 - 0.11433071498151929859e-2_f64 * t54925 - 0.25724410870841842183e-2_f64 * t11859 * t3117 * t15780 * t15609 - 0.12862205435420921092e-2_f64 * t11859 * t3117 * t4893 * t54931 + 0.64311027177104605458e-3_f64 * t11875 * t3117 * t4893 * t54936 - 0.95275595817932748826e-4_f64 * t54943;
    (t54931, t54936, t54945)
}
