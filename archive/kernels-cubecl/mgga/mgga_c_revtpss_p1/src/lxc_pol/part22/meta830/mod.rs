//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta830 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2950;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta830<F: Float>(t1882: F, t4056: F, t2682: F, t4000: F, t5677: F, t820: F, t13985: F, t46740: F, t1872: F, t3924: F, t9816: F, t9818: F, t13848: F, t47274: F, t9956: F, t13878: F, t9765: F, t13869: F, t3989: F, t2661: F, t5608: F, t9840: F, t9934: F) -> (F, F, F, F, F, F, F, F) {
        let (t48475, t48486, t48488, t48494) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2950::<F>(t1882, t4056, t2682, t4000, t5677, t820, t13985, t46740, t1872, t3924, t9816, t9818);
        let (t48498, t48508, t48510, t48514) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2951::<F>(t13848, t47274, t9816, t9956, t13878, t9765, t13869, t3989, t2661, t5608, t9840, t9934);
    (t48475, t48486, t48488, t48494, t48498, t48508, t48510, t48514)
}
