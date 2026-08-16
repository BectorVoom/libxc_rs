//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1834;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta507(t7769: f64, t886: f64, t25317: f64, t225: f64, t27265: f64, t1579: f64, t231: f64, t836: f64, t25392: f64, t7048: f64, t7071: f64, t7759: f64, t25399: f64, t4481: f64, t1580: f64, t213: f64, t25322: f64, t25362: f64, t25364: f64, t25366: f64, t25368: f64, t25371: f64, t25379: f64, t25391: f64, t257: f64, t27199: f64, t7070: f64, t7079: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27299, t27300, t27303, t27312, t27313, t27316, t27317, t27322) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1834(t7769, t886, t25317, t225, t27265, t1579, t231, t836, t25392, t7048, t7071, t7759);
        let (t27325, t27329) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1835(t25399, t4481, t1580, t213, t25322, t25362, t25364, t25366, t25368, t25371, t25379, t25391, t257, t27199, t27300, t27303, t27313, t27317, t27322, t7070, t7079);
    (t27299, t27300, t27303, t27312, t27313, t27316, t27317, t27322, t27325, t27329)
}
