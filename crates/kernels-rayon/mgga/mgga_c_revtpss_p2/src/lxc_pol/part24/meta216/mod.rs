//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk961;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta216(t11465: f64, t315: f64, t11132: f64, t11337: f64, t3010: f64, t963: f64, t3013: f64, t323: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t11466, t11479, t11480, t11506) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk961(t11465, t315, t11132, t11337, t3010, t963);
        let (t11507, t11509) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk962(t11506, t315, t3013, t323);
    (t11466, t11479, t11480, t11506, t11507, t11509)
}
