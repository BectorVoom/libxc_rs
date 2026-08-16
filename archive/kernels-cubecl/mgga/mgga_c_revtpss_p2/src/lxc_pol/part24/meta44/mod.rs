//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta44 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk301;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk302;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk303;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk304;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk305;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk306;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta44<F: Float>(t902: F, t139: F, t221: F, t346: F, t345: F, t220: F, t344: F, t44: F, t124: F, t65: F, t270: F, t271: F, t905: F, t225: F, t994: F, t366: F, t196: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t997, t1009, t1010) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk301::<F>(t902, t139, t221, t346, t345, t220, t344);
        let t1011 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk302::<F>(t1010, t44);
        let t1012 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk303::<F>(t124, t65);
        let t1014 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk304::<F>(t270, t271);
        let (t1015, t1024) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk305::<F>(t1014, t905, t225, t994);
        let t1025 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk306::<F>(t1024, t366);
        let (t1031, t1032) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk307::<F>(t196);
    (t997, t1009, t1010, t1011, t1012, t1014, t1015, t1024, t1025, t1031, t1032)
}
