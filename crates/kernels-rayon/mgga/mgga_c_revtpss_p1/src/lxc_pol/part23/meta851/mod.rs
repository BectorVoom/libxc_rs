//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta851 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2735;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta851(t3718: f64, t44546: f64, t6689: f64, t1222: f64, t17240: f64, t20318: f64, t1263: f64, t372: f64, t6622: f64, t17241: f64, t5373: f64, t17654: f64, t20766: f64, t56756: f64, t17693: f64, t20937: f64, t20310: f64, t20306: f64, t12772: f64, t21156: f64, t3625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71294, t71297, t71300) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2735(t3718, t44546, t6689, t1222, t17240, t20318, t1263, t372, t6622);
        let (t71320, t71329, t71341, t71373, t71377, t71400) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2736(t17241, t5373, t17654, t20766, t56756, t17693, t20937, t1222, t17240, t20310, t20306, t12772, t21156, t3625);
    (t71294, t71297, t71300, t71320, t71329, t71341, t71373, t71377, t71400)
}
