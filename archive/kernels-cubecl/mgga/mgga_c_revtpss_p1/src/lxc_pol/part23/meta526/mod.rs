//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2046;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta526<F: Float>(t20800: F, t5465: F, t5480: F, t3302: F, t471: F, t1214: F, t20795: F, t1287: F, t21298: F, t5464: F, t21164: F, t20900: F, t487: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21465, t21468, t21471, t21472, t21473, t21480, t21483, t21484, t21491, t21495) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2046::<F>(t20800, t5465, t5480, t3302, t471, t1214, t20795, t1287, t21298, t5464, t21164, t20900, t487);
    (t21465, t21468, t21471, t21472, t21473, t21480, t21483, t21484, t21491, t21495)
}
