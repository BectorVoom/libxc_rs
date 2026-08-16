//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta948 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3133;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3134;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3135;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta948<F: Float>(t24252: F, t300: F, t1198: F, t1765: F, t68609: F, t16784: F, t6552: F, t20384: F, t5192: F, t24498: F, t3531: F, t20400: F, t5202: F, t24480: F, t6556: F, t1179: F, t1188: F, t1196: F, t81998: F, t1187: F, t24375: F, t45187: F, t45190: F, t1189: F, t24493: F, t82060: F, t81635: F, t81638: F, t81641: F, t81646: F, t81649: F, t81653: F, t81656: F, t81660: F, t82119: F, t82385: F, t82386: F, t82388: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t82391, t82394, t82396, t82398, t82400, t82402) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3133::<F>(t24252, t300, t1198, t1765, t68609, t16784, t6552, t20384, t5192, t24498, t3531, t20400, t5202);
        let (t82404, t82406, t82410, t82415) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3134::<F>(t24480, t3531, t16784, t6556, t1179, t1188, t1196, t81998, t1187, t24375, t45187, t45190);
        let (t82418, t82419) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3135::<F>(t1189, t1196, t24493, t82060, t82394, t82396, t82398, t82400, t82402, t82404, t82406, t82410, t82415);
        let t82422 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3136::<F>(t81635, t81638, t81641, t81646, t81649, t81653, t81656, t81660, t82119, t82385, t82386, t82388, t82391, t82419);
    (t82391, t82394, t82396, t82398, t82400, t82402, t82404, t82406, t82410, t82415, t82418, t82422)
}
