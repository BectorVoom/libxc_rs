//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta114 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk740;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk741;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk742;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk743;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk744;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta114<F: Float>(t2846: F, t960: F, t964: F, t320: F, t963: F, t315: F, t2904: F, t323: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2974, t2982, t2985, t2986) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk740::<F>(t2846, t960, t964, t320, t963);
        let t2987 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk741::<F>(t2986, t315);
        let (t2994, t3001, t3010) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk742::<F>(t2846, t2904, t963);
        let t3011 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk743::<F>(t3010);
        let t3012 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk744::<F>(t3011, t315);
        let (t3013, t3014) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk745::<F>(t323);
    (t2974, t2982, t2985, t2986, t2987, t2994, t3001, t3010, t3011, t3012, t3013, t3014)
}
