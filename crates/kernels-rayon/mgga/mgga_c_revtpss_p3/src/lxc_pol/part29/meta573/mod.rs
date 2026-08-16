//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1920;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta573(t27253: f64, t9775: f64, t14833: f64, t240: f64, t2661: f64, t7043: f64, t14853: f64, t7045: f64, t14857: f64, t25234: f64, t25240: f64, t2710: f64, t4371: f64, t10744: f64, t4353: f64, t7028: f64, t14701: f64, t92955: f64, t14707: f64, t25270: f64, t241: f64, t820: f64, t93060: f64, t14896: f64, t4447: f64, t92951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98964, t98968, t98970, t98972, t98976) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1920(t27253, t9775, t14833, t240, t2661, t7043, t14853, t7045, t14857, t25234, t25240, t2710, t4371);
        let (t98979, t98983, t98985, t98989, t98991) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1921(t10744, t4353, t7028, t14701, t92955, t14707, t25270, t241, t820, t93060, t14896, t4447, t92951);
    (t98964, t98968, t98970, t98972, t98976, t98979, t98983, t98985, t98989, t98991)
}
