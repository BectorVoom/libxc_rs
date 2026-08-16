//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta876 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3041;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta876(t10542: f64, t14563: f64, t14519: f64, t2470: f64, t2798: f64, t231: f64, t51049: f64, t2782: f64, t2797: f64, t14663: f64, t686: f64, t72: f64, t4522: f64, t874: f64, t9288: f64, t1573: f64, t40317: f64, t14587: f64, t39608: f64, t10069: f64, t14496: f64, t14524: f64, t39575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51429, t51434, t51436, t51438, t51442) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3041(t10542, t14563, t14519, t2470, t2798, t231, t51049, t2782, t2797, t14663, t686, t72);
        let (t51445, t51452, t51460, t51470, t51483) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3042(t4522, t874, t9288, t1573, t40317, t14587, t2782, t39608, t10069, t14496, t14524, t39575);
    (t51429, t51434, t51436, t51438, t51442, t51445, t51452, t51460, t51470, t51483)
}
