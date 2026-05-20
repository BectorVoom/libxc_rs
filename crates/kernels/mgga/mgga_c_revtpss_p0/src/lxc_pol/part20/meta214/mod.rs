//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk997;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk998;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta214<F: Float>(t10430: F, t10432: F, t10435: F, t10438: F, t10442: F, t10444: F, t10469: F, t10489: F, t198: F, t765: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t2828: F, t886: F, t2770: F, t2435: F, t2445: F, t2441: F, t9303: F, t10115: F, t258: F, t2453: F, t2464: F, t2438: F, t138: F, t2434: F, t123: F, t2465: F, t213: F, t2760: F, t215: F, t231: F, t268: F, t836: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t10493 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk997::<F>(t10430, t10432, t10435, t10438, t10442, t10444, t10469, t10489, t198, t765, t9278, t9308, t9316, t9329, t9333);
        let (t10495, t10498, t10501, t10503, t10504, t10505) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk998::<F>(t2828, t886, t2770, t2435, t2445, t2441, t9303, t10115, t258, t2453, t2464, t2438);
        let (t10506, t10507, t10510, t10511, t10513, t10518) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk999::<F>(t10505, t138, t10504, t2434, t886, t123, t2465, t213, t2760, t215, t231, t268, t836);
    (t10493, t10495, t10498, t10501, t10503, t10504, t10506, t10507, t10510, t10511, t10513, t10518)
}
