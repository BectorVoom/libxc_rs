//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2093;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2094;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2095;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2096;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta496<F: Float>(t11710: F, t4787: F, t3091: F, t245: F, t4890: F, t3088: F, t3317: F, t1065: F, t1668: F, t372: F, t12131: F, t3095: F, t4823: F, t3096: F, t1087: F, t11773: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15682, t15684, t15687, t15688) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2093::<F>(t11710, t4787, t3091, t245, t4890, t3088);
        let t15689 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2094::<F>(t15688, t3317);
        let (t15690, t15691) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2095::<F>(t1065, t1668, t372);
        let (t15692, t15693, t15696) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2096::<F>(t12131, t3095, t15691, t372, t4823);
        let (t15697, t15700) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2097::<F>(t15696, t3096, t1087, t11773);
    (t15682, t15684, t15687, t15688, t15689, t15690, t15691, t15692, t15693, t15696, t15697, t15700)
}
