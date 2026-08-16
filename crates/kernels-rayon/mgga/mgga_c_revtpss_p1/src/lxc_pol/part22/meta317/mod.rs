//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1760;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta317(t10578: f64, t2630: f64, t2629: f64, t9866: f64, t9575: f64, t9572: f64, t177: f64, t2390: f64, t762: f64, t760: f64, t9419: f64, t2516: f64, t2523: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10579, t10582, t10584, t10586, t10587, t10588, t10592, t10593) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1760(t10578, t2630, t2629, t9866, t9575, t9572, t177, t2390, t762, t760, t9419, t2516, t2523);
    (t10579, t10582, t10584, t10586, t10587, t10588, t10592, t10593)
}
