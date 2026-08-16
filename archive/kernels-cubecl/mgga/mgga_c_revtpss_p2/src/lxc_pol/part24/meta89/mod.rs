//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta89 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk523;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk524;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk525;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta89<F: Float>(t159: F, t243: F, t216: F, t2712: F, t785: F, t225: F, t826: F, t849: F, t820: F, t823: F, t843: F, t241: F, t72: F, t853: F, t245: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2729, t2730, t2735) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk523::<F>(t159, t243, t216, t2712, t785);
        let (t2736, t2737, t2739, t2741) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk524::<F>(t225, t2735, t826, t849, t820, t823, t843);
        let t2745 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk525::<F>(t241, t820, t823);
        let (t2746, t2747) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk526::<F>(t72, t853, t245);
    (t2729, t2730, t2735, t2736, t2737, t2739, t2741, t2745, t2746, t2747)
}
