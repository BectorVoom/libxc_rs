//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1920;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta573<F: Float>(t27253: F, t9775: F, t14833: F, t240: F, t2661: F, t7043: F, t14853: F, t7045: F, t14857: F, t25234: F, t25240: F, t2710: F, t4371: F, t10744: F, t4353: F, t7028: F, t14701: F, t92955: F, t14707: F, t25270: F, t241: F, t820: F, t93060: F, t14896: F, t4447: F, t92951: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98964, t98968, t98970, t98972, t98976) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1920::<F>(t27253, t9775, t14833, t240, t2661, t7043, t14853, t7045, t14857, t25234, t25240, t2710, t4371);
        let (t98979, t98983, t98985, t98989, t98991) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1921::<F>(t10744, t4353, t7028, t14701, t92955, t14707, t25270, t241, t820, t93060, t14896, t4447, t92951);
    (t98964, t98968, t98970, t98972, t98976, t98979, t98983, t98985, t98989, t98991)
}
