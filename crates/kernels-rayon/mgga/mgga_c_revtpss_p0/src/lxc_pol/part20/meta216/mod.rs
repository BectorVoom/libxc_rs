//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1001;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta216(t231: f64, t281: f64, t68: f64, t836: f64, t10535: f64, t2783: f64, t860: f64, t786: f64, t2801: f64, t2645: f64, t268: f64, t675: f64, t2798: f64, t10430: f64, t10432: f64, t10435: f64, t10438: f64, t10442: f64, t10444: f64, t10469: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10538, t10539, t10541, t10542, t10543, t10547) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1001(t231, t281, t68, t836, t10535, t2783, t860, t786, t2801, t2645, t268, t675);
        let (t10548, t10550) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1002(t10547, t2798, t10430, t10432, t10435, t10438, t10442, t10444, t10469, t9278, t9308, t9316, t9329);
    (t10538, t10539, t10541, t10542, t10543, t10547, t10548, t10550)
}
