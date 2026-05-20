//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1420;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta379<F: Float>(t1646: F, t3056: F, t225: F, t3106: F, t4817: F, t11710: F, t4787: F, t3091: F, t245: F, t4890: F, t3088: F, t3317: F, t1065: F, t1668: F, t372: F, t4823: F, t1087: F, t11773: F, t1062: F, t4857: F, t11986: F, t1592: F, t247: F, t1063: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15669, t15670, t15675, t15684, t15687, t15688, t15689) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1420::<F>(t1646, t3056, t225, t3106, t4817, t11710, t4787, t3091, t245, t4890, t3088, t3317);
        let (t15691, t15696, t15700, t15707, t15712) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1421::<F>(t1065, t1668, t372, t4823, t1087, t11773, t1062, t4857, t11986, t1592, t247, t1063);
    (t15669, t15670, t15675, t15684, t15687, t15688, t15689, t15691, t15696, t15700, t15707, t15712)
}
