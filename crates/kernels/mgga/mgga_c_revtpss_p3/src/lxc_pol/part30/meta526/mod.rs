//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1939;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1940;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta526<F: Float>(t4181: F, t603: F, t4187: F, t1493: F, t644: F, t77: F, t4173: F, t607: F, t7705: F, t1497: F, t1927: F, t1470: F, t2247: F, t7239: F, t7898: F, t197: F, t530: F, t2013: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t28116, t28119, t28133, t28141, t28147, t28150, t28154) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1939::<F>(t4181, t603, t4187, t1493, t644, t77, t4173, t607, t7705, t1497, t1927, t1470, t2247);
        let (t28165, t28166, t28167) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1940::<F>(t7239, t7898, t197, t530, t2013);
    (t28116, t28119, t28133, t28141, t28147, t28150, t28154, t28165, t28166, t28167)
}
