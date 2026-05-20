//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta96<F: Float>(t2113: F, t2115: F, t572: F, t573: F, t10: F, t17: F, t576: F, t580: F, t15: F, t22: F, t11: F, t14: F) -> (F, F, F, F, F) {
        let (t2118, t2219, t2221, t2223, t2224) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk590::<F>(t2113, t2115, t572, t573, t10, t17, t576, t580, t15, t22, t11, t14);
    (t2118, t2219, t2221, t2223, t2224)
}
