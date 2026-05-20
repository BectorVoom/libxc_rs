//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta18 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk141;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk142;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk143;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk144;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk145;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta18<F: Float>(t335: F, t136: F, t44: F, t271: F, t221: F, t65: F, t225: F, t342: F, t336: F, t73: F, t293: F, t328: F, t330: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t343, t344) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk141::<F>(t335, t136);
        let t345 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk142::<F>(t344, t44);
        let t346 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk143::<F>(t271);
        let (t348, t351, t354, t355, t357) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk144::<F>(t221, t346, t65, t225, t342, t336, t73, t293, t328, t330);
        let (t358, t359) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk145::<F>(t357);
        let t360 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk146::<F>(sigma0);
    (t343, t344, t345, t346, t348, t351, t354, t355, t357, t358, t359, t360)
}
