//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta80 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk479;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk480;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk481;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk482;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk483;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta80<F: Float>(t136: F, t826: F, t737: F, t744: F, t185: F) -> (F, F, F, F, F, F, F) {
        let t2485 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk479::<F>(t136, t826);
        let t2490 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk480::<F>(t737);
        let t2491 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk481::<F>(t2490);
        let t2492 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk482::<F>(t744);
        let (t2494, t2495) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk483::<F>(t185);
        let t2496 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk484::<F>(t2491, t2492, t2495);
    (t2485, t2490, t2491, t2492, t2494, t2495, t2496)
}
