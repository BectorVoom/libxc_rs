//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1804;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta478<F: Float>(t4135: F, t4147: F, t2034: F, t2014: F, t10416: F, t1936: F, t13435: F, t2322: F, t7002: F, t13440: F, t5523: F, t112: F, t239: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25802, t25803, t25804, t25812, t25814, t25816, t25818, t25820, t25821) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1804::<F>(t4135, t4147, t2034, t2014, t10416, t1936, t13435, t2322, t7002, t13440, t5523, t112, t239);
    (t25802, t25803, t25804, t25812, t25814, t25816, t25818, t25820, t25821)
}
