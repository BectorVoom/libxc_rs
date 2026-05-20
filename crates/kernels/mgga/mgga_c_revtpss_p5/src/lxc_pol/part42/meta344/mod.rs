//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1149;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1150;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta344<F: Float>(t3172: F, t4802: F, t1063: F, t4807: F, t11922: F, t4911: F, t3115: F, t1032: F, t4743: F, t1040: F, t11921: F, t247: F, t4757: F, t4837: F, t1659: F, t3105: F, t1062: F, t4797: F, t1660: F, t3201: F, t1058: F, t4798: F, t15127: F, t15125: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15774, t15776, t15796, t15817, t15827) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1149::<F>(t3172, t4802, t1063, t4807, t11922, t4911, t3115, t1032, t4743, t1040, t11921, t247, t4757);
        let (t15829, t15830, t15850, t15862, t15865, t15874, t15875) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1150::<F>(t15827, t4837, t1659, t3105, t1062, t4797, t1660, t3201, t1058, t4798, t15127, t15125);
    (t15774, t15776, t15796, t15817, t15829, t15830, t15850, t15862, t15865, t15874, t15875)
}
