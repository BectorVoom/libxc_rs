//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1180;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1181;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1182;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1183;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta338<F: Float>(t11341: F, t23470: F, t141: F, t22671: F, t905: F, t930: F, t11142: F, t128: F, t11150: F, t22688: F, t2850: F, t2852: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23471, t23472, t23474) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1180::<F>(t11341, t23470, t141, t22671, t905);
        let (t23475, t23476, t23478, t23479) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1181::<F>(t23474, t930, t141, t11142, t23470, t128);
        let t23481 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1182::<F>(t11150, t22688);
        let (t23482, t23483) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1183::<F>(t23481, t2850, t128);
        let t23485 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1184::<F>(t22688, t2852);
    (t23471, t23472, t23474, t23475, t23476, t23478, t23479, t23481, t23482, t23483, t23485)
}
