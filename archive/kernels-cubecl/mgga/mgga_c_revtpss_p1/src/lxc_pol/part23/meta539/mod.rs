//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2085;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta539<F: Float>(t22046: F, t3936: F, t9835: F, t1414: F, t21969: F, t828: F, t221: F, t3979: F, t6816: F, t3978: F, t3989: F, t6880: F, t22025: F, t543: F, t3992: F, t2661: F, t1370: F, t13779: F, t13781: F, t13797: F, t1410: F, t22038: F, t22041: F, t22044: F, t5671: F, t9735: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22048, t22052, t22056, t22057, t22059) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2085::<F>(t22046, t3936, t9835, t1414, t21969, t828, t221, t3979, t6816, t3978, t3989, t6880);
        let (t22061, t22062, t22063, t22065) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2086::<F>(t22025, t543, t3992, t2661, t1370, t13779, t13781, t13797, t1410, t22038, t22041, t22044, t22048, t22052, t22057, t22059, t5671, t9735);
    (t22048, t22052, t22056, t22057, t22059, t22061, t22062, t22063, t22065)
}
