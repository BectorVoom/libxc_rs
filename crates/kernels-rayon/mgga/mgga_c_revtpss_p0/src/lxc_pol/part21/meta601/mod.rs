//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta601(t231: f64, t268: f64, t2798: f64, t793: f64, t836: f64, t215: f64, t2722: f64, t2645: f64, t4366: f64, t10529: f64, t2782: f64, t14545: f64, t251: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t39581, t39583, t39586, t39588, t39590, t39595, t39597) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2327(t231, t268, t2798, t793, t836, t215, t2722, t2645, t4366, t10529, t2782, t14545, t251);
    (t39581, t39583, t39586, t39588, t39590, t39595, t39597)
}
