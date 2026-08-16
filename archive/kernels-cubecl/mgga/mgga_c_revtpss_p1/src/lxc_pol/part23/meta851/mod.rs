//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta851 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2735;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta851<F: Float>(t3718: F, t44546: F, t6689: F, t1222: F, t17240: F, t20318: F, t1263: F, t372: F, t6622: F, t17241: F, t5373: F, t17654: F, t20766: F, t56756: F, t17693: F, t20937: F, t20310: F, t20306: F, t12772: F, t21156: F, t3625: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t71294, t71297, t71300) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2735::<F>(t3718, t44546, t6689, t1222, t17240, t20318, t1263, t372, t6622);
        let (t71320, t71329, t71341, t71373, t71377, t71400) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2736::<F>(t17241, t5373, t17654, t20766, t56756, t17693, t20937, t1222, t17240, t20310, t20306, t12772, t21156, t3625);
    (t71294, t71297, t71300, t71320, t71329, t71341, t71373, t71377, t71400)
}
