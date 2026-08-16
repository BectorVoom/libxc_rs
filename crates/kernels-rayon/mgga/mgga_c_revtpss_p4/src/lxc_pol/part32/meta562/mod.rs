//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1882;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta562(t14701: f64, t92955: f64, t241: f64, t820: f64, t93060: f64, t4447: f64, t92951: f64, t14727: f64, t25227: f64, t2661: f64, t4430: f64, t93034: f64, t92991: f64, t14861: f64, t1565: f64, t93066: f64, t25222: f64, t4345: f64, t4349: f64, t93072: f64, t14673: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98983, t98988, t98991, t99000, t99002) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1882(t14701, t92955, t241, t820, t93060, t4447, t92951, t14727, t25227, t2661, t4430, t93034);
        let (t99004, t99006, t99009, t99011, t99013, t99019) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1883(t92991, t14861, t25227, t2661, t1565, t93066, t25222, t4345, t4349, t93072, t14673, t92955);
    (t98983, t98988, t98991, t99000, t99002, t99004, t99006, t99009, t99011, t99013, t99019)
}
