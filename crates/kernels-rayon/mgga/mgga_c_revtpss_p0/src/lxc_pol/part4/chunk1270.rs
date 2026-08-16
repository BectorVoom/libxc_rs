//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1270/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1270(t13396: f64, t4806: f64, t1042: f64, t1651: f64, t3075: f64, t247: f64, t3116: f64, t1066: f64, t15193: f64, t1062: f64, t4797: f64, t1047: f64, t1063: f64, t1068: f64, t11991: f64, t15817: f64, t15823: f64, t15829: f64, t15830: f64, t1675: f64, t3136: f64, t3157: f64, t3177: f64, t3188: f64, t4831: f64, t4834: f64, t4837: f64, t4879: f64) -> (f64, f64) {
    let t15833 = t4806 * t13396;
    let t15834 = t1042 * t15833;
    let t15837 = t1651 * t3075;
    let t15839 = t247 * t3116 * t15837;
    let t15847 = t247 * t1066 * t15193;
    let t15850 = t4797 * t1062;
    let t15855 = 0.42874018118069736972e-3_f64 * t15817 * t1047 + 0.21437009059034868486e-3_f64 * t4879 * t3136 + 0.42874018118069736972e-3_f64 * t15823 * t3157 + t15829 - 0.15244095330869239812e-2_f64 * t15830 * t1068 + 0.47637797908966374414e-3_f64 * t1063 * t15834 + 0.42874018118069736972e-3_f64 * t4837 * t15839 + 0.14291339372689912324e-3_f64 * t11991 * t1675 + 0.28582678745379824648e-3_f64 * t3188 * t4831 + 0.14291339372689912324e-3_f64 * t1063 * t15847 + 0.28582678745379824648e-3_f64 * t15850 * t1068 + 0.14291339372689912324e-3_f64 * t4834 * t3177;
    (t15837, t15855)
}
