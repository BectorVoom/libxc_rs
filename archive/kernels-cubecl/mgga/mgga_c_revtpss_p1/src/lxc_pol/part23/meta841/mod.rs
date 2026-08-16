//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta841 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2716;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta841<F: Float>(t1261: F, t12879: F, t247: F, t6425: F, t12772: F, t21227: F, t3625: F, t21021: F, t21007: F, t44425: F, t21222: F, t5340: F, t21101: F, t3707: F, t17608: F, t5292: F, t17547: F, t5265: F, t20906: F, t3172: F, t17416: F, t5391: F, t21272: F, t3636: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t70032, t70039, t70044, t70064, t70076) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2716::<F>(t1261, t12879, t247, t6425, t12772, t21227, t3625, t21021, t21007, t44425, t21222, t5340);
        let (t70082, t70088, t70091, t70102, t70112, t70114) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2717::<F>(t21101, t3707, t17608, t5292, t17547, t5265, t1261, t20906, t3172, t17416, t5391, t21272, t3636);
    (t70032, t70039, t70044, t70064, t70076, t70082, t70088, t70091, t70102, t70112, t70114)
}
