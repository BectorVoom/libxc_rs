//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1594;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1595;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta422(t1032: f64, t4743: f64, t1040: f64, t1647: f64, t3140: f64, t3149: f64, t11921: f64, t247: f64, t4757: f64, t4837: f64, t1659: f64, t3105: f64, t13396: f64, t4806: f64, t1042: f64, t1651: f64, t3075: f64, t3116: f64, t1066: f64, t15193: f64, t1062: f64, t4797: f64, t1047: f64, t1063: f64, t1068: f64, t11991: f64, t1675: f64, t3136: f64, t3157: f64, t3177: f64, t3188: f64, t4831: f64, t4834: f64, t4879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15816, t15817, t15822, t15823, t15827, t15829, t15830) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1594(t1032, t4743, t1040, t1647, t3140, t3149, t11921, t247, t4757, t4837, t1659, t3105);
        let (t15834, t15837, t15839, t15847, t15855) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1595(t13396, t4806, t1042, t1651, t3075, t247, t3116, t1066, t15193, t1062, t4797, t1047, t1063, t1068, t11991, t15817, t15823, t15829, t15830, t1675, t3136, t3157, t3177, t3188, t4831, t4834, t4837, t4879);
    (t15816, t15822, t15827, t15834, t15837, t15839, t15847, t15855)
}
