//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta200<F: Float>(t10288: F, t2237: F, t592: F, t2236: F, t3: F, t25: F, t88: F, t89: F, t90: F, t29: F, t46: F, t47: F) -> (F, F, F, F, F, F, F, F) {
        let (t10289, t10291, t10292, t10293, t10295, t10308, t10309, t10355) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk934::<F>(t10288, t2237, t592, t2236, t3, t25, t88, t89, t90, t29, t46, t47);
    (t10289, t10291, t10292, t10293, t10295, t10308, t10309, t10355)
}
