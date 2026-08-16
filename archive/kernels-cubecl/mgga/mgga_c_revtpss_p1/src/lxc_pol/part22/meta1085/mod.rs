//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1085 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1085<F: Float>(t1456: F, t1464: F, t22533: F, t22571: F, t3: F, t4154: F, t47730: F, t575: F, t60607: F, t60620: F, t60624: F, t60629: F, t6951: F, t75716: F, t75720: F, t75801: F) -> F {
        let tv4rho42 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3937::<F>(t1456, t1464, t22533, t22571, t3, t4154, t47730, t575, t60607, t60620, t60624, t60629, t6951, t75716, t75720, t75801);
    tv4rho42
}
