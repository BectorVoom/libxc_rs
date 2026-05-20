//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1216;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta352<F: Float>(t23911: F, t6100: F, t3092: F, t6092: F, t11703: F, t6096: F, t1011: F, t1063: F, t11737: F, t15618: F, t15712: F, t15732: F, t15750: F, t19786: F, t19827: F, t19867: F, t19883: F, t23874: F, t23878: F, t23886: F, t23892: F, t23900: F, t23904: F, t23908: F, t3091: F, t3127: F, t4834: F, t4892: F, t4899: F, t6268: F, t6331: F) -> (F, F, F, F, F, F, F) {
        let (t23912, t23913, t23916, t23917, t23920, t23921, t23926) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1216::<F>(t23911, t6100, t3092, t6092, t11703, t6096, t1011, t1063, t11737, t15618, t15712, t15732, t15750, t19786, t19827, t19867, t19883, t23874, t23878, t23886, t23892, t23900, t23904, t23908, t3091, t3127, t4834, t4892, t4899, t6268, t6331);
    (t23912, t23913, t23916, t23917, t23920, t23921, t23926)
}
