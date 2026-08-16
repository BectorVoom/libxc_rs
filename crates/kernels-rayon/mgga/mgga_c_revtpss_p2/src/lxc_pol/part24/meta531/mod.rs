//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1567;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1568;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta531(t3603: f64, t6622: f64, t1284: f64, t24698: f64, t487: f64, t83107: f64, t22648: f64, t602: f64, t1469: f64, t1486: f64, t72: f64, t23042: f64, t3915: f64, t686: f64, t22970: f64, t9680: f64, t22453: f64, t49471: f64, t1358: f64, t212: f64, t22964: f64, t689: f64, t13848: f64, t22893: f64, t47274: f64, t9816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t84645, t84859, t84952, t84967, t85037, t85161, t85475) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1567(t3603, t6622, t1284, t24698, t487, t83107, t22648, t602, t1469, t1486, t72, t23042, t3915, t686);
        let (t85480, t85484, t85509, t85514) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1568(t22970, t686, t72, t9680, t22453, t49471, t1358, t212, t22964, t689, t13848, t22893, t47274, t9816);
    (t84645, t84859, t84952, t84967, t85037, t85161, t85475, t85480, t85484, t85509, t85514)
}
