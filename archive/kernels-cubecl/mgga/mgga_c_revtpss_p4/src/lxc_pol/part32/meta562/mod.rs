//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1882;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta562<F: Float>(t14701: F, t92955: F, t241: F, t820: F, t93060: F, t4447: F, t92951: F, t14727: F, t25227: F, t2661: F, t4430: F, t93034: F, t92991: F, t14861: F, t1565: F, t93066: F, t25222: F, t4345: F, t4349: F, t93072: F, t14673: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98983, t98988, t98991, t99000, t99002) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1882::<F>(t14701, t92955, t241, t820, t93060, t4447, t92951, t14727, t25227, t2661, t4430, t93034);
        let (t99004, t99006, t99009, t99011, t99013, t99019) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1883::<F>(t92991, t14861, t25227, t2661, t1565, t93066, t25222, t4345, t4349, t93072, t14673, t92955);
    (t98983, t98988, t98991, t99000, t99002, t99004, t99006, t99009, t99011, t99013, t99019)
}
