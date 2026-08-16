//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta395 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1974;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1975;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta395<F: Float>(t13847: F, t13848: F, t5675: F, t13845: F, t3924: F, t5673: F, t5674: F, t5609: F, t9794: F, t9793: F, t13817: F, t13821: F, t13826: F, t13832: F, t13834: F, t13841: F, t1410: F, t3934: F, t5671: F, t9739: F, t9742: F, t9745: F, t1353: F, t5591: F, t4012: F, t828: F, t1868: F, t3889: F, t221: F, t5627: F, t9921: F, t3978: F, t13583: F, t13585: F, t13593: F, t13599: F, t13612: F, t13615: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13850, t13851, t13854, t13858, t13860) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1974::<F>(t13847, t13848, t5675, t13845, t3924, t5673, t5674, t5609, t9794, t9793, t13817, t13821, t13826, t13832, t13834, t13841, t1410, t3934, t5671, t9739, t9742, t9745);
        let t13867 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1975::<F>(t1353, t5591);
        let (t13869, t13872, t13874, t13878, t13880, t13881) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1976::<F>(t13867, t4012, t828, t1868, t3889, t221, t5627, t9921, t3978, t13583, t13585, t13593, t13599, t13612, t13615, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t13850, t13851, t13854, t13858, t13860, t13867, t13869, t13872, t13874, t13878, t13880, t13881)
}
