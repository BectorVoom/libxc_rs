//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1216;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta352(t23911: f64, t6100: f64, t3092: f64, t6092: f64, t11703: f64, t6096: f64, t1011: f64, t1063: f64, t11737: f64, t15618: f64, t15712: f64, t15732: f64, t15750: f64, t19786: f64, t19827: f64, t19867: f64, t19883: f64, t23874: f64, t23878: f64, t23886: f64, t23892: f64, t23900: f64, t23904: f64, t23908: f64, t3091: f64, t3127: f64, t4834: f64, t4892: f64, t4899: f64, t6268: f64, t6331: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t23912, t23913, t23916, t23917, t23920, t23921, t23926) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1216(t23911, t6100, t3092, t6092, t11703, t6096, t1011, t1063, t11737, t15618, t15712, t15732, t15750, t19786, t19827, t19867, t19883, t23874, t23878, t23886, t23892, t23900, t23904, t23908, t3091, t3127, t4834, t4892, t4899, t6268, t6331);
    (t23912, t23913, t23916, t23917, t23920, t23921, t23926)
}
