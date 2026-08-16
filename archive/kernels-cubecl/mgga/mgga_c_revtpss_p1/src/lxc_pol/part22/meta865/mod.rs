//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta865 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3019;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta865<F: Float>(t14860: F, t2661: F, t2662: F, t837: F, t2646: F, t4352: F, t14652: F, t4416: F, t14663: F, t221: F, t2484: F, t2485: F, t10811: F, t14919: F, t14904: F, t14923: F, t241: F, t40322: F, t820: F, t2659: F, t2783: F, t816: F, t808: F, t853: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t50732, t50736, t50740, t50744, t50748) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3019::<F>(t14860, t2661, t2662, t837, t2646, t4352, t14652, t4416, t14663, t221, t2484, t2485);
        let (t50752, t50754, t50757, t50768, t50769) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3020::<F>(t10811, t14919, t14904, t14923, t241, t40322, t820, t2659, t2783, t816, t808, t853);
    (t50732, t50736, t50740, t50744, t50748, t50752, t50754, t50757, t50768, t50769)
}
