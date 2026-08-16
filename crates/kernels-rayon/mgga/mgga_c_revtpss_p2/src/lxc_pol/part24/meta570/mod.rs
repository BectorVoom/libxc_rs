//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta570(t448: f64, t90305: f64, t90317: f64, t300: f64, t24480: f64, t5192: f64, t6438: f64, t44091: f64, t44093: f64, t16840: f64, t24221: f64, t1150: f64, t12248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t90319, t90321, t90323, t90324, t90327, t90329, t90332) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1748(t448, t90305, t90317, t300, t24480, t5192, t6438, t44091, t44093, t16840, t24221, t1150, t12248);
    (t90319, t90321, t90323, t90324, t90327, t90329, t90332)
}
