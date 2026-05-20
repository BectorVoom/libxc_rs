//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta470 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2161;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2162;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2163;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2164;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2165;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta470<F: Float>(t12131: F, t3095: F, t15691: F, t372: F, t4823: F, t3096: F, t1087: F, t11773: F, t4801: F, t4181: F, t4786: F, t1062: F, t4857: F, t11986: F, t1592: F, t247: F, t1063: F, t11940: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15692, t15693, t15696) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2161::<F>(t12131, t3095, t15691, t372, t4823);
        let (t15697, t15700) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2162::<F>(t15696, t3096, t1087, t11773);
        let (t15701, t15702) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2163::<F>(t372, t4801, t4181, t4786);
        let (t15703, t15707) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2164::<F>(t15701, t15702, t1062, t4857);
        let (t15711, t15712, t15716) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2165::<F>(t11986, t1592, t247, t1063, t1062, t11940);
    (t15692, t15693, t15696, t15697, t15700, t15701, t15702, t15703, t15707, t15711, t15712, t15716)
}
