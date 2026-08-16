//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta115 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk667;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk668;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk669;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk670;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk671;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta115<F: Float>(t3196: F, t366: F, t371: F, t373: F, t676: F, t367: F, t225: F, t3057: F, t3059: F, t372: F, t1024: F, t1053: F, t1026: F, t127: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3197, t3201) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk667::<F>(t3196, t366, t371, t373, t676);
        let (t3203, t3204) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk668::<F>(t3201, t367, t225, t3057);
        let (t3205, t3206, t3208) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk669::<F>(t3204, t366, t3059, t373, t371, t372);
        let t3211 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk670::<F>(t1024, t1053);
        let t3215 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk671::<F>(t1026, t127, t371);
    (t3197, t3201, t3203, t3204, t3205, t3206, t3208, t3211, t3215)
}
