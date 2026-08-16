//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta948 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3133;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3134;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3135;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta948(t24252: f64, t300: f64, t1198: f64, t1765: f64, t68609: f64, t16784: f64, t6552: f64, t20384: f64, t5192: f64, t24498: f64, t3531: f64, t20400: f64, t5202: f64, t24480: f64, t6556: f64, t1179: f64, t1188: f64, t1196: f64, t81998: f64, t1187: f64, t24375: f64, t45187: f64, t45190: f64, t1189: f64, t24493: f64, t82060: f64, t81635: f64, t81638: f64, t81641: f64, t81646: f64, t81649: f64, t81653: f64, t81656: f64, t81660: f64, t82119: f64, t82385: f64, t82386: f64, t82388: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82391, t82394, t82396, t82398, t82400, t82402) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3133(t24252, t300, t1198, t1765, t68609, t16784, t6552, t20384, t5192, t24498, t3531, t20400, t5202);
        let (t82404, t82406, t82410, t82415) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3134(t24480, t3531, t16784, t6556, t1179, t1188, t1196, t81998, t1187, t24375, t45187, t45190);
        let (t82418, t82419) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3135(t1189, t1196, t24493, t82060, t82394, t82396, t82398, t82400, t82402, t82404, t82406, t82410, t82415);
        let t82422 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3136(t81635, t81638, t81641, t81646, t81649, t81653, t81656, t81660, t82119, t82385, t82386, t82388, t82391, t82419);
    (t82391, t82394, t82396, t82398, t82400, t82402, t82404, t82406, t82410, t82415, t82418, t82422)
}
