//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1161/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1161<F: Float>(t380: F, t994: F, t16088: F, t606: F, t999: F, t4578: F, t3092: F, t905: F, t1045: F, t15691: F, t11774: F, t11917: F, t11924: F, t11938: F, t11952: F, t11954: F, t11956: F, t11965: F, t16078: F, t16081: F, t16084: F, t16089: F, t16091: F, t3115: F) -> (F, F, F) {
    let t16094 = t994 * t380;
    let t16095 = t16094 * t16088;
    let t16096 = t606 * t999;
    let t16097 = t4578 * t16096;
    let t16098 = t3092 * t16097;
    let t16101 = t999 * t905;
    let t16102 = t16101 * t606;
    let t16103 = t1045 * t16102;
    let t16104 = t15691 * t16103;
    let t16114 = -0.21437009059034868486e-3 * t3115 * t16078 + 0.12862205435420921092e-2 * t16081 * t16084 + 0.57165357490759649296e-3 * t16089 * t16091 + 0.57165357490759649296e-3 * t16095 * t16098 - 0.28582678745379824648e-3 * t11774 * t16104 - 0.14291339372689912324e-3 * t11917 - 0.28582678745379824648e-3 * t11924 + 0.28582678745379824648e-3 * t11938 - 0.14291339372689912324e-3 * t11952 - 0.15244095330869239812e-2 * t11954 - 0.95275595817932748826e-4 * t11956 + 0.48272968547752592739e-2 * t11965;
    (t16095, t16096, t16114)
}
