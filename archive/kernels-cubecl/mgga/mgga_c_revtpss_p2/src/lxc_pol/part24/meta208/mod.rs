//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk946;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk947;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta208<F: Float>(t821: F, t235: F, t239: F, t820: F, t231: F, t2723: F, t2710: F, t826: F, t9732: F, t234: F, t2735: F, t2699: F, t798: F, t159: F, t853: F, t216: F, t2729: F, t794: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10866, t10867, t10868) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk946::<F>(t821, t235);
        let (t10870, t10871) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk947::<F>(t10868, t239, t820, t231, t2723);
        let (t10885, t10886, t10890, t10899, t10900, t10905) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk948::<F>(t2710, t826, t9732, t234, t2735, t2699, t798, t159, t853, t216, t2729, t794);
    (t10866, t10867, t10868, t10870, t10871, t10885, t10886, t10890, t10899, t10900, t10905)
}
