//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta476<F: Float>(t1063: F, t20054: F, t19572: F, t4894: F, t3117: F, t4900: F, t11774: F, t15926: F, t20040: F, t20046: F, t20051: F, t3106: F, t3188: F, t4892: F, t4899: F, t4912: F, t6323: F, t6327: F, t6331: F) -> (F, F, F, F, F, F) {
        let (t20055, t20065, t20066, t20069, t20070, t20073) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1928::<F>(t1063, t20054, t19572, t4894, t3117, t4900, t11774, t15926, t20040, t20046, t20051, t3106, t3188, t4892, t4899, t4912, t6323, t6327, t6331);
    (t20055, t20065, t20066, t20069, t20070, t20073)
}
