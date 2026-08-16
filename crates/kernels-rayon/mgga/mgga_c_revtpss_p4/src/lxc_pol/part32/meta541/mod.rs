//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta541(t25375: f64, t95628: f64, t136: f64, t137: f64, t2061: f64, t10505: f64, t93377: f64, t7406: f64, t9288: f64, t7064: f64, t10073: f64, t25308: f64, t26554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t95722, t95725, t95726, t95727, t95730, t95732, t95740) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1852(t25375, t95628, t136, t137, t2061, t10505, t93377, t7406, t9288, t7064, t10073, t25308, t26554);
    (t95722, t95725, t95726, t95727, t95730, t95732, t95740)
}
