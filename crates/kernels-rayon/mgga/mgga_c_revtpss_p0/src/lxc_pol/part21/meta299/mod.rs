//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta299(t10578: f64, t2630: f64, t2629: f64, t9866: f64, t9575: f64, t9572: f64, t177: f64, t2390: f64, t762: f64, t10575: f64, t10577: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10579, t10580, t10582, t10584, t10586, t10587, t10588, t10589, t10590) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1549(t10578, t2630, t2629, t9866, t9575, t9572, t177, t2390, t762, t10575, t10577, t9514, t9517, t9521, t9524);
    (t10579, t10580, t10582, t10584, t10586, t10587, t10588, t10589, t10590)
}
