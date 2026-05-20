//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1709;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta467<F: Float>(t3957: F, t6884: F, t124: F, t21969: F, t800: F, t6850: F, t9744: F, t125: F, t6861: F, t3936: F, t9835: F, t1414: F, t828: F, t221: F, t3979: F, t6816: F, t3978: F, t3989: F, t6880: F, t22025: F, t543: F, t3992: F, t2661: F, t1370: F, t13779: F, t13781: F, t13797: F, t1410: F, t5671: F, t9735: F) -> (F, F, F, F, F, F, F) {
        let (t22038, t22041, t22044, t22046, t22048, t22052) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1709::<F>(t3957, t6884, t124, t21969, t800, t6850, t9744, t125, t6861, t3936, t9835, t1414, t828);
        let (t22056, t22061, t22065) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1710::<F>(t221, t3979, t6816, t3978, t3989, t6880, t22025, t543, t3992, t2661, t1370, t13779, t13781, t13797, t1410, t22038, t22041, t22044, t22048, t22052, t5671, t9735);
    (t22041, t22046, t22048, t22052, t22056, t22061, t22065)
}
