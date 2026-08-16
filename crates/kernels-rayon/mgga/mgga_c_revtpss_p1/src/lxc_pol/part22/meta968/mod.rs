//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta968 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta968(t50852: f64, t50856: f64, t18562: f64, t2516: f64, t2496: f64, t18305: f64, t2258: f64, t4401: f64, t14325: f64, t18306: f64, t5825: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t61292, t61293, t61295, t61297, t61300, t61302, t61303) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3233(t50852, t50856, t18562, t2516, t2496, t18305, t2258, t4401, t14325, t18306, t5825, t749);
    (t61292, t61293, t61295, t61297, t61300, t61302, t61303)
}
