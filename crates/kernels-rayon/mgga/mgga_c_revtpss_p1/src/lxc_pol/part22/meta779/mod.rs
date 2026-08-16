//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta779 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta779(t43813: f64, t43816: f64, t3475: f64, t426: f64, t3478: f64, t3488: f64, t3520: f64, t1175: f64, t12552: f64, t43752: f64, t439: f64, t3519: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45106, t45107, t45157, t45159, t45168, t45174, t45177, t45186) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2869(t43813, t43816, t3475, t426, t3478, t3488, t3520, t1175, t12552, t43752, t439, t3519);
    (t45106, t45107, t45157, t45159, t45168, t45174, t45177, t45186)
}
