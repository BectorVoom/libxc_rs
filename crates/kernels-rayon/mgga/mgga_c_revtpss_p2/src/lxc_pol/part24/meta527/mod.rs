//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1560;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta527(t1032: f64, t1246: f64, t24698: f64, t1222: f64, t140: f64, t24830: f64, t17471: f64, t24236: f64, t24679: f64, t369: f64, t467: f64, t475: f64, t5390: f64, t6601: f64, t21177: f64, t5362: f64, t1235: f64, t127: f64, t24634: f64, t371: f64, t20842: f64, t5327: f64, t17396: f64, t20926: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83607, t83699, t83719, t83725) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1560(t1032, t1246, t24698, t1222, t140, t24830, t17471, t24236, t24679, t369, t467, t475);
        let (t83728, t83731, t83735, t83748, t83751) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1561(t5390, t6601, t21177, t5362, t1235, t127, t24634, t371, t20842, t5327, t17396, t20926);
    (t83607, t83699, t83719, t83725, t83728, t83731, t83735, t83748, t83751)
}
