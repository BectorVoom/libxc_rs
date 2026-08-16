//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta548(t7289: f64, t96282: f64, t26277: f64, t94776: f64, t25950: f64, t26292: f64, t25904: f64, t96245: f64, t94471: f64, t94473: f64, t94476: f64, t94483: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96284, t96287, t96289, t96298, t96321, t96322, t96323, t96326) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1863(t7289, t96282, t26277, t94776, t25950, t26292, t25904, t96245, t94471, t94473, t94476, t94483);
    (t96284, t96287, t96289, t96298, t96321, t96322, t96323, t96326)
}
