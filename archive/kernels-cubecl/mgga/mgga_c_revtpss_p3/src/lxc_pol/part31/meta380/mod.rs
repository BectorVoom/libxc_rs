//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta380<F: Float>(t3172: F, t4802: F, t1063: F, t4807: F, t11922: F, t4911: F, t3115: F, t1032: F, t4743: F, t1040: F, t11921: F, t247: F, t4757: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15772, t15774, t15775, t15776, t15794, t15796, t15816, t15817, t15827) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1417::<F>(t3172, t4802, t1063, t4807, t11922, t4911, t3115, t1032, t4743, t1040, t11921, t247, t4757);
    (t15772, t15774, t15775, t15776, t15794, t15796, t15816, t15817, t15827)
}
