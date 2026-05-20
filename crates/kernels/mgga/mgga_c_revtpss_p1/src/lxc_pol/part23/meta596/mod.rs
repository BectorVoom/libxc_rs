//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta596<F: Float>(t19501: F, t23898: F, t3092: F, t6266: F, t19611: F, t357: F, t4781: F, t6100: F, t6092: F, t11703: F, t6096: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23899, t23900, t23903, t23904, t23907, t23908, t23911, t23912, t23913, t23916, t23917, t23920) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2239::<F>(t19501, t23898, t3092, t6266, t19611, t357, t4781, t6100, t6092, t11703, t6096);
    (t23899, t23900, t23903, t23904, t23907, t23908, t23911, t23912, t23913, t23916, t23917, t23920)
}
