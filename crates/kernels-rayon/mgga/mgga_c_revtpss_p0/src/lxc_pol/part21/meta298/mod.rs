//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1547;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta298(t10565: f64, t158: f64, t755: f64, t9586: f64, t2619: f64, t2622: f64, t10552: f64, t10554: f64, t10557: f64, t10560: f64, t10562: f64, t10564: f64, t9333: f64, t9394: f64, t2390: f64, t72: f64, t757: f64, t2629: f64, t9863: f64, t123: f64, t752: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10566, t10568, t10569, t10570, t10571) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1547(t10565, t158, t755, t9586, t2619, t2622, t10552, t10554, t10557, t10560, t10562, t10564, t9333, t9394);
        let (t10573, t10574, t10575, t10577, t10578) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1548(t2390, t72, t757, t2629, t9863, t123, t752);
    (t10566, t10568, t10569, t10570, t10571, t10573, t10574, t10575, t10577, t10578)
}
