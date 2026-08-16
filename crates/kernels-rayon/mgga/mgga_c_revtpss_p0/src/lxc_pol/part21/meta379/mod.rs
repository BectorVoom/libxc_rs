//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1790;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta379(t12378: f64, t448: f64, t300: f64, t12295: f64, t12292: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t422: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12379, t12381, t12382, t12393) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1790(t12378, t448, t300, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
        let (t12395, t12397, t12408) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1791(t12393, t422, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
    (t12379, t12381, t12382, t12393, t12395, t12397, t12408)
}
