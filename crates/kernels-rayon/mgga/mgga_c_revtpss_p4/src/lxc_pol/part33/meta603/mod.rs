//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta603(t26894: f64, t26921: f64, t1294: f64, t471: f64, t355: f64, t1210: f64, t3627: f64, t5457: f64, t29193: f64, t1203: f64, t5464: f64, t3566: f64, t7627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96927, t96929, t96953, t96954, t96979, t96982, t96986, t97019) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2027(t26894, t26921, t1294, t471, t355, t1210, t3627, t5457, t29193, t1203, t5464, t3566, t7627);
    (t96927, t96929, t96953, t96954, t96979, t96982, t96986, t97019)
}
