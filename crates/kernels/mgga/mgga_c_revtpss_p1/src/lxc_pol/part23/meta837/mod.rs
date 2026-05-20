//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta837 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2708;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta837<F: Float>(t1160: F, t20597: F, t20447: F, t3435: F, t3565: F, t6563: F, t225: F, t1261: F, t12879: F, t247: F, t6429: F, t11262: F, t1247: F, t6624: F, t21102: F, t3704: F, t21094: F, t3172: F, t5384: F, t17361: F, t5274: F, t5261: F, t5390: F, t12915: F, t20703: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t69565, t69591, t69636, t69637, t69661, t69668) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2708::<F>(t1160, t20597, t20447, t3435, t3565, t6563, t225, t1261, t12879, t247, t6429, t11262, t1247, t6624);
        let (t69674, t69698, t69700, t69710, t69719) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2709::<F>(t21102, t3704, t21094, t3172, t5384, t17361, t5274, t5261, t5390, t12915, t20703, t247);
    (t69565, t69591, t69636, t69637, t69661, t69668, t69674, t69698, t69700, t69710, t69719)
}
