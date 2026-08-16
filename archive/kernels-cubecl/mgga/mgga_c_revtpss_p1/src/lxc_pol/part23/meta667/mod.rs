//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2399;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta667<F: Float>(t11240: F, t3144: F, t42646: F, t11239: F, t989: F, t11629: F, t11874: F, t16048: F, t12046: F, t15905: F, t994: F, t1011: F, t1016: F, t2438: F, t1014: F, t11150: F, t1003: F, t11735: F, t221: F, t345: F, t346: F, t624: F, t11858: F, t1065: F, t215: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42648, t42668, t42669, t42675, t42690, t42716) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2399::<F>(t11240, t3144, t42646, t11239, t989, t11629, t11874, t16048, t12046, t15905, t994, t1011, t1016, t2438);
        let (t42731, t42740, t42745, t42765, t42778) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2400::<F>(t1014, t11150, t1003, t11735, t221, t345, t346, t624, t11858, t16048, t1065, t215);
    (t42648, t42668, t42669, t42675, t42690, t42716, t42731, t42740, t42745, t42765, t42778)
}
