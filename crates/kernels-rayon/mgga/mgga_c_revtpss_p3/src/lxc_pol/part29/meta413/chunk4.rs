//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1508/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1508(t15691: f64, t16103: f64, t11774: f64, t11917: f64, t11924: f64, t11938: f64, t11952: f64, t11954: f64, t11956: f64, t11965: f64, t16078: f64, t16081: f64, t16084: f64, t16089: f64, t16091: f64, t16095: f64, t16098: f64, t3115: f64) -> f64 {
    let t16104 = t15691 * t16103;
    let t16114 = -0.21437009059034868486e-3_f64 * t3115 * t16078 + 0.12862205435420921092e-2_f64 * t16081 * t16084 + 0.57165357490759649296e-3_f64 * t16089 * t16091 + 0.57165357490759649296e-3_f64 * t16095 * t16098 - 0.28582678745379824648e-3_f64 * t11774 * t16104 - 0.14291339372689912324e-3_f64 * t11917 - 0.28582678745379824648e-3_f64 * t11924 + 0.28582678745379824648e-3_f64 * t11938 - 0.14291339372689912324e-3_f64 * t11952 - 0.15244095330869239812e-2_f64 * t11954 - 0.95275595817932748826e-4_f64 * t11956 + 0.48272968547752592739e-2_f64 * t11965;
    t16114
}
