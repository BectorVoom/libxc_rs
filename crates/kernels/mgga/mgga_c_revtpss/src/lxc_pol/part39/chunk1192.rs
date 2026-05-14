//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1192/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1192<F: Float>(t15837: F, t247: F, t3116: F, t1066: F, t15193: F, t1062: F, t4797: F, t1047: F, t1063: F, t1068: F, t11991: F, t15817: F, t15823: F, t15829: F, t15830: F, t15834: F, t1675: F, t3136: F, t3157: F, t3177: F, t3188: F, t4831: F, t4834: F, t4837: F, t4879: F) -> (F,) {
    let t15839 = t247 * t3116 * t15837;
    let t15847 = t247 * t1066 * t15193;
    let t15850 = t4797 * t1062;
    let t15855 = 0.42874018118069736972e-3 * t15817 * t1047 + 0.21437009059034868486e-3 * t4879 * t3136 + 0.42874018118069736972e-3 * t15823 * t3157 + t15829 - 0.15244095330869239812e-2 * t15830 * t1068 + 0.47637797908966374414e-3 * t1063 * t15834 + 0.42874018118069736972e-3 * t4837 * t15839 + 0.14291339372689912324e-3 * t11991 * t1675 + 0.28582678745379824648e-3 * t3188 * t4831 + 0.14291339372689912324e-3 * t1063 * t15847 + 0.28582678745379824648e-3 * t15850 * t1068 + 0.14291339372689912324e-3 * t4834 * t3177;
    (t15855,)
}
