//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2066;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta625<F: Float>(t14857: F, t25234: F, t25240: F, t2710: F, t4371: F, t10744: F, t4353: F, t7028: F, t14701: F, t92955: F, t241: F, t820: F, t93060: F, t4447: F, t92951: F, t14727: F, t25227: F, t2661: F, t4430: F, t93034: F, t14861: F, t1565: F, t93066: F, t25222: F, t4345: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98973, t98976, t98979, t98984, t98988) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2066::<F>(t14857, t25234, t25240, t2710, t4371, t10744, t4353, t7028, t14701, t92955, t241, t820, t93060);
        let (t98992, t99001, t99002, t99007, t99009, t99011) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2067::<F>(t4447, t92951, t14727, t25227, t2661, t4430, t93034, t14861, t1565, t93066, t25222, t4345);
    (t98973, t98976, t98979, t98984, t98988, t98992, t99001, t99002, t99007, t99009, t99011)
}
