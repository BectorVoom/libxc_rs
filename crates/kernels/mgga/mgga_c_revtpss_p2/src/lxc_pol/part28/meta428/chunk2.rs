//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1613/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1613<F: Float>(t15691: F, t16103: F, t11774: F, t11917: F, t11924: F, t11938: F, t11952: F, t11954: F, t11956: F, t11965: F, t16078: F, t16081: F, t16084: F, t16089: F, t16091: F, t16095: F, t16098: F, t3115: F) -> (F, F) {
    let t16104 = t15691 * t16103;
    let t16114 = -F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t16078 + F::cast_from(0.12862205435420921092e-2_f64) * t16081 * t16084 + F::cast_from(0.57165357490759649296e-3_f64) * t16089 * t16091 + F::cast_from(0.57165357490759649296e-3_f64) * t16095 * t16098 - F::cast_from(0.28582678745379824648e-3_f64) * t11774 * t16104 - F::cast_from(0.14291339372689912324e-3_f64) * t11917 - F::cast_from(0.28582678745379824648e-3_f64) * t11924 + F::cast_from(0.28582678745379824648e-3_f64) * t11938 - F::cast_from(0.14291339372689912324e-3_f64) * t11952 - F::cast_from(0.15244095330869239812e-2_f64) * t11954 - F::cast_from(0.95275595817932748826e-4_f64) * t11956 + F::cast_from(0.48272968547752592739e-2_f64) * t11965;
    (t16104, t16114)
}
