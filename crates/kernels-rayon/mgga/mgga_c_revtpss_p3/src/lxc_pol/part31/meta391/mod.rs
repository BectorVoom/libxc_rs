//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta391(t16712: f64, t300: f64, t5155: f64, t16710: f64, t16708: f64, t1130: f64, t5060: f64, t1719: f64, t3432: f64, t5101: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16713, t16784, t16797, t16798, t16820, t16821, t16822, t16835, t16840, t16868, t16869, t16873) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1432(t16712, t300, t5155, t16710, t16708, t1130, t5060, t1719, t3432, t5101, t698);
    (t16713, t16784, t16797, t16798, t16820, t16821, t16822, t16835, t16840, t16868, t16869, t16873)
}
