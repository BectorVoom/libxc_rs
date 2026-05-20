//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2029;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta599<F: Float>(t94395: F, t97688: F, t94649: F, t1892: F, t786: F, t25877: F, t25881: F, t2028: F, t25931: F, t14224: F, t689: F, t25894: F, t25875: F, t122: F, t3916: F, t72: F, t7910: F, t25895: F, t2022: F, t9990: F, t1426: F, t7911: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97690, t97698, t97699, t97700, t97702, t97703, t97705, t97707) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2029::<F>(t94395, t97688, t94649, t1892, t786, t25877, t25881, t2028, t25931, t14224, t689, t25894);
        let (t97719, t97732, t97734, t97764, t97783) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2030::<F>(t25875, t97703, t97705, t122, t3916, t72, t7910, t25895, t2022, t9990, t1426, t786, t7911);
    (t97690, t97698, t97699, t97700, t97702, t97707, t97719, t97732, t97734, t97764, t97783)
}
