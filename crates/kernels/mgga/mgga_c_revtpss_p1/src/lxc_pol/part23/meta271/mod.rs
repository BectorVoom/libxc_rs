//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1482;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1483;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta271<F: Float>(t2438: F, t886: F, t138: F, t10504: F, t2434: F, t123: F, t2465: F, t215: F, t231: F, t268: F, t836: F, t2798: F, t251: F, t4503: F, t786: F, t2453: F, t2797: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10505, t10506, t10507, t10509, t10510, t10511, t10518, t10519) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1482::<F>(t2438, t886, t138, t10504, t2434, t123, t2465, t215, t231, t268, t836, t2798);
        let t10529 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1483::<F>(t251, t4503);
        let (t10530, t10535) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1484::<F>(t10529, t786, t2453, t2797);
    (t10505, t10506, t10507, t10509, t10510, t10511, t10518, t10519, t10529, t10530, t10535)
}
