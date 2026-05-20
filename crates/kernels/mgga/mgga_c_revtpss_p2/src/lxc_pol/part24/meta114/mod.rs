//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta114 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk644;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta114<F: Float>(t1317: F, t1333: F, t1340: F, t2516: F, t2496: F, t2626: F, t1412: F, t73: F, t1389: F, t1408: F, t2736: F, t1425: F, t560: F) -> (F, F, F, F, F, F, F, F) {
        let (t4027, t4035, t4037, t4042, t4049, t4062, t4064, t4075) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk644::<F>(t1317, t1333, t1340, t2516, t2496, t2626, t1412, t73, t1389, t1408, t2736, t1425, t560);
    (t4027, t4035, t4037, t4042, t4049, t4062, t4064, t4075)
}
