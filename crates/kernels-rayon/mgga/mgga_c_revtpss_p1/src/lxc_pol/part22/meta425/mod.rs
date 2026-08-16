//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2038;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2039;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta425(t10871: f64, t2722: f64, t2777: f64, t4518: f64, t2439: f64, t2470: f64, t4499: f64, t2798: f64, t1568: f64, t2783: f64, t786: f64, t2801: f64, t10533: f64, t10539: f64, t10543: f64, t10548: f64, t10645: f64, t10647: f64, t10651: f64, t10655: f64, t14546: f64, t2646: f64, t2724: f64, t2754: f64, t4494: f64, t4504: f64, t4514: f64, t4526: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t14547 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2038(t10871, t2722);
        let (t14557, t14558, t14563, t14564, t14567, t14568) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2039(t2777, t4518, t2439, t2470, t4499, t2798, t1568, t2783, t786);
        let (t14570, t14572) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2040(t14568, t2801, t10533, t10539, t10543, t10548, t10645, t10647, t10651, t10655, t14546, t14547, t14558, t14564, t2646, t2724, t2754, t4494, t4504, t4514, t4526, t820);
    (t14547, t14557, t14558, t14563, t14564, t14567, t14568, t14570, t14572)
}
