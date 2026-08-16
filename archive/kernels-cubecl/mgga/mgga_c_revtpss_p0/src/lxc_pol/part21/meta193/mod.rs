//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta193 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1170;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1171;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1172;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1173;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1174;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta193<F: Float>(t2850: F, t4574: F, t128: F, t1469: F, t2857: F, t606: F, t904: F, t4186: F, t905: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4575, t4576) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1170::<F>(t2850, t4574, t128);
        let t4578 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1171::<F>(t1469, t2857);
        let t4579 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1172::<F>(t4578, t606);
        let (t4580, t4581) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1173::<F>(t4579, t904, t128);
        let t4583 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1174::<F>(t4186, t905);
        let (t4584, t4585) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1175::<F>(t4583, t904, t128);
    (t4575, t4576, t4578, t4579, t4580, t4581, t4583, t4584, t4585)
}
