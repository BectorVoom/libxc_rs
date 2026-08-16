//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1524;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta509(t23492: f64, t698: f64, t23471: f64, t23495: f64, t23510: f64, t23507: f64, t23475: f64, t23663: f64, t914: f64, t23798: f64, t945: f64, t23811: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77663, t77667, t77736, t77804, t77806, t77858, t78097, t78108, t78111) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1524(t23492, t698, t23471, t23495, t23510, t23507, t23475, t23663, t914, t23798, t945, t23811, t964);
    (t77663, t77667, t77736, t77804, t77806, t77858, t78097, t78108, t78111)
}
