//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk913;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta212(t3992: f64, t5609: f64, t2661: f64, t1414: f64, t5591: f64, t828: f64, t1413: f64, t1868: f64, t547: f64, t807: f64, t221: f64, t3979: f64, t3978: f64, t1885: f64, t3930: f64, t1353: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5610, t5611, t5614, t5617, t5618, t5619, t5622) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk913(t3992, t5609, t2661, t1414, t5591, t828, t1413, t1868, t547, t807, t221, t3979);
        let (t5623, t5625, t5627) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk914(t3978, t5622, t1885, t3930, t1353, t1868);
    (t5610, t5611, t5614, t5617, t5618, t5619, t5622, t5623, t5625, t5627)
}
