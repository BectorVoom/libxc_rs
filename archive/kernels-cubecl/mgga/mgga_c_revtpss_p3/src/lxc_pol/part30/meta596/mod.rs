//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta596<F: Float>(t1032: F, t3727: F, t2148: F, t1276: F, t3140: F, t26894: F, t26921: F, t1294: F, t471: F, t355: F, t1204: F, t7627: F) -> (F, F, F, F, F, F, F) {
        let (t96873, t96874, t96910, t96927, t96928, t96929, t96933) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2057::<F>(t1032, t3727, t2148, t1276, t3140, t26894, t26921, t1294, t471, t355, t1204, t7627);
    (t96873, t96874, t96910, t96927, t96928, t96929, t96933)
}
