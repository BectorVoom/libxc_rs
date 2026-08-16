//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1149;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1150;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta344(t3172: f64, t4802: f64, t1063: f64, t4807: f64, t11922: f64, t4911: f64, t3115: f64, t1032: f64, t4743: f64, t1040: f64, t11921: f64, t247: f64, t4757: f64, t4837: f64, t1659: f64, t3105: f64, t1062: f64, t4797: f64, t1660: f64, t3201: f64, t1058: f64, t4798: f64, t15127: f64, t15125: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15774, t15776, t15796, t15817, t15827) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1149(t3172, t4802, t1063, t4807, t11922, t4911, t3115, t1032, t4743, t1040, t11921, t247, t4757);
        let (t15829, t15830, t15850, t15862, t15865, t15874, t15875) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1150(t15827, t4837, t1659, t3105, t1062, t4797, t1660, t3201, t1058, t4798, t15127, t15125);
    (t15774, t15776, t15796, t15817, t15829, t15830, t15850, t15862, t15865, t15874, t15875)
}
