//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3006/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3006<F: Float>(t1043: F, t43116: F, t3075: F, t4900: F, t1063: F, t11986: F, t247: F, t4583: F, t11859: F, t11875: F, t15609: F, t15703: F, t15780: F, t3117: F, t3120: F, t4893: F, t4899: F, t53923: F, t54907: F, t54909: F, t54914: F, t54916: F, t54919: F, t54925: F) -> (F, F, F) {
    let t54931 = t43116 * t1043;
    let t54936 = t4900 * t3075;
    let t54943 = t1063 * t247 * t11986 * t4583;
    let t54945 = -F::cast_from(0.28582678745379824648e-3_f64) * t54907 - F::cast_from(0.64311027177104605458e-3_f64) * t4899 * t3117 * t54909 * t4900 - F::cast_from(0.85748036236139473944e-3_f64) * t54914 + F::cast_from(0.68598428988911579154e-2_f64) * t54916 * t3120 - F::cast_from(0.85748036236139473944e-3_f64) * t54919 + F::cast_from(0.91464571985215438873e-2_f64) * t53923 * t15703 - F::cast_from(0.11433071498151929859e-2_f64) * t54925 - F::cast_from(0.25724410870841842183e-2_f64) * t11859 * t3117 * t15780 * t15609 - F::cast_from(0.12862205435420921092e-2_f64) * t11859 * t3117 * t4893 * t54931 + F::cast_from(0.64311027177104605458e-3_f64) * t11875 * t3117 * t4893 * t54936 - F::cast_from(0.95275595817932748826e-4_f64) * t54943;
    (t54931, t54936, t54945)
}
