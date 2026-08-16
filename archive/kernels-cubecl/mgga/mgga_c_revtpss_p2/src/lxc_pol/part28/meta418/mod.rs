//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1584;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta418<F: Float>(t15648: F, t373: F, t371: F, t372: F, t4742: F, t993: F, t225: F, t366: F, t3224: F, t4845: F, t127: F, t4852: F, t1025: F, t1646: F, t3056: F, t3106: F, t4817: F, t1028: F, t11644: F, t11649: F, t11783: F, t1665: F, t3208: F, t3211: F, t3220: F, t4854: F, t4858: F) -> (F, F, F, F, F, F, F) {
        let (t15651, t15654, t15655, t15656, t15662, t15666) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1584::<F>(t15648, t373, t371, t372, t4742, t993, t225, t366, t3224, t4845, t127, t4852);
        let (t15669, t15670, t15676) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1585::<F>(t1025, t15666, t1646, t3056, t225, t366, t3106, t4817, t1028, t11644, t11649, t11783, t15651, t15656, t15662, t1665, t3208, t3211, t3220, t3224, t4854, t4858);
    (t15651, t15654, t15655, t15666, t15669, t15670, t15676)
}
