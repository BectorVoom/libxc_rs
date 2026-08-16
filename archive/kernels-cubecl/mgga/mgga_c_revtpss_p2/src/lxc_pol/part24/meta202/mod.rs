//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta202 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta202<F: Float>(t2441: F, t9303: F, t10115: F, t258: F, t2453: F, t2464: F, t251: F, t4503: F, t786: F, t2797: F, t760: F, t9323: F) -> (F, F, F, F, F, F, F) {
        let (t10501, t10503, t10504, t10529, t10530, t10535, t10552) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk936::<F>(t2441, t9303, t10115, t258, t2453, t2464, t251, t4503, t786, t2797, t760, t9323);
    (t10501, t10503, t10504, t10529, t10530, t10535, t10552)
}
