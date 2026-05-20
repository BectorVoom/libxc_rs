//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1560;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta527<F: Float>(t1032: F, t1246: F, t24698: F, t1222: F, t140: F, t24830: F, t17471: F, t24236: F, t24679: F, t369: F, t467: F, t475: F, t5390: F, t6601: F, t21177: F, t5362: F, t1235: F, t127: F, t24634: F, t371: F, t20842: F, t5327: F, t17396: F, t20926: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t83607, t83699, t83719, t83725) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1560::<F>(t1032, t1246, t24698, t1222, t140, t24830, t17471, t24236, t24679, t369, t467, t475);
        let (t83728, t83731, t83735, t83748, t83751) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1561::<F>(t5390, t6601, t21177, t5362, t1235, t127, t24634, t371, t20842, t5327, t17396, t20926);
    (t83607, t83699, t83719, t83725, t83728, t83731, t83735, t83748, t83751)
}
