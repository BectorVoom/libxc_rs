//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1417;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1418;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta452<F: Float>(t2662: F, t268: F, t40689: F, t4353: F, t40710: F, t4349: F, t1558: F, t231: F, t40406: F, t685: F, t72: F, t826: F, t10760: F, t40763: F, t2710: F, t4371: F, t9732: F, t4398: F, t9323: F, t4302: F, t9586: F, t9425: F, t10565: F, t1532: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50381, t50385, t50436) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1417::<F>(t2662, t268, t40689, t4353, t40710, t4349, t1558, t231, t40406, t685, t72, t826);
        let (t50611, t50703, t50852, t50856, t50888, t50892) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1418::<F>(t10760, t40763, t4353, t2710, t4371, t9732, t4398, t9323, t4302, t9586, t9425, t10565, t1532);
    (t50381, t50385, t50436, t50611, t50703, t50852, t50856, t50888, t50892)
}
