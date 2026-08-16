//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta567(t93282: f64, t93317: f64, t786: f64, t860: f64, t25410: f64, t25413: f64, t7064: f64, t93150: f64, t25375: f64, t93311: f64, t122: f64, t7048: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t93318, t93320, t93321, t93322, t93324, t93326, t93329) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2027(t93282, t93317, t786, t860, t25410, t25413, t7064, t93150, t25375, t93311, t122, t7048, t72);
    (t93318, t93320, t93321, t93322, t93324, t93326, t93329)
}
