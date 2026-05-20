//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta728 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2496;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta728<F: Float>(t49476: F, t1358: F, t2439: F, t5710: F, t785: F, t1426: F, t5711: F, t786: F, t14100: F, t9686: F, t1353: F, t198: F, t10199: F, t1514: F, t2289: F, t4264: F, t10227: F, t97: F, t10241: F, t105: F, t4288: F, t4398: F, t9372: F, t1469: F, t2608: F, t4401: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49477, t49480, t49503, t49513, t49541) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2496::<F>(t49476, t1358, t2439, t5710, t785, t1426, t5711, t786, t14100, t9686, t1353, t198);
        let (t49698, t49701, t49777, t49787, t49818, t49866, t49876) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2497::<F>(t10199, t1514, t2289, t4264, t10227, t97, t10241, t105, t4288, t4398, t9372, t1469, t2608, t4401, t606);
    (t49477, t49480, t49503, t49513, t49541, t49698, t49701, t49777, t49787, t49818, t49866, t49876)
}
