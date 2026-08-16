//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2066;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta625(t14857: f64, t25234: f64, t25240: f64, t2710: f64, t4371: f64, t10744: f64, t4353: f64, t7028: f64, t14701: f64, t92955: f64, t241: f64, t820: f64, t93060: f64, t4447: f64, t92951: f64, t14727: f64, t25227: f64, t2661: f64, t4430: f64, t93034: f64, t14861: f64, t1565: f64, t93066: f64, t25222: f64, t4345: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98973, t98976, t98979, t98984, t98988) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2066(t14857, t25234, t25240, t2710, t4371, t10744, t4353, t7028, t14701, t92955, t241, t820, t93060);
        let (t98992, t99001, t99002, t99007, t99009, t99011) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2067(t4447, t92951, t14727, t25227, t2661, t4430, t93034, t14861, t1565, t93066, t25222, t4345);
    (t98973, t98976, t98979, t98984, t98988, t98992, t99001, t99002, t99007, t99009, t99011)
}
