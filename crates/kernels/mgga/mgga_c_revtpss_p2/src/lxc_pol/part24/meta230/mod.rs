//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta230 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta230<F: Float>(t1242: F, t474: F, t11243: F, t479: F, t13036: F, t3603: F, t471: F) -> (F, F, F, F, F, F, F) {
        let (t13037, t13038, t13039, t13040, t13041, t13042, t13045) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk988::<F>(t1242, t474, t11243, t479, t13036, t3603, t471);
    (t13037, t13038, t13039, t13040, t13041, t13042, t13045)
}
