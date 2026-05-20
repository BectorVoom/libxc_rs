//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta320<F: Float>(t2440: F, t887: F, t2439: F, t866: F, t225: F, t2771: F, t886: F, t2461: F, t2471: F, t788: F, t9288: F, t787: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11003, t11004, t11006, t11007, t11008, t11009, t11010, t11013, t11015, t11017) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1596::<F>(t2440, t887, t2439, t866, t225, t2771, t886, t2461, t2471, t788, t9288, t787);
    (t11003, t11004, t11006, t11007, t11008, t11009, t11010, t11013, t11015, t11017)
}
