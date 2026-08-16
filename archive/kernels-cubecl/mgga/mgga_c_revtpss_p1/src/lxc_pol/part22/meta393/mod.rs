//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1967;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1968;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta393<F: Float>(t13790: F, t13791: F, t13789: F, t3957: F, t5690: F, t1873: F, t9741: F, t5651: F, t808: F, t9736: F, t241: F, t820: F, t9991: F, t3923: F, t9994: F, t5673: F, t5674: F, t5697: F, t9962: F, t5701: F, t13778: F, t13779: F, t13781: F, t13786: F, t3934: F, t5671: F, t9735: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13792, t13793, t13797, t13798, t13800, t13801, t13804) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1967::<F>(t13790, t13791, t13789, t3957, t5690, t1873, t9741, t5651, t808, t9736, t241, t820, t9991);
        let t13805 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1968::<F>(t3923, t9994);
        let (t13807, t13810, t13813, t13814) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1969::<F>(t13805, t5673, t5674, t5697, t9962, t5701, t13778, t13779, t13781, t13786, t13793, t13797, t13798, t13801, t13804, t3934, t5671, t9735);
    (t13792, t13793, t13797, t13798, t13800, t13801, t13804, t13805, t13807, t13810, t13813, t13814)
}
