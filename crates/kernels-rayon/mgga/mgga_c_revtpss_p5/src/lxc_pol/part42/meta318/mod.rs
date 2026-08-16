//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1093;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta318(t550: f64, t5658: f64, t543: f64, t3992: f64, t2661: f64, t5610: f64, t9775: f64, t1889: f64, t9779: f64, t828: f64, t9954: f64, t1398: f64, t1868: f64, t3935: f64, t1882: f64, t4003: f64, t3957: f64, t5690: f64, t1873: f64, t9741: f64, t5651: f64, t808: f64, t9736: f64, t241: f64, t820: f64, t9991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13778, t13779, t13781, t13783, t13784) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1093(t550, t5658, t543, t3992, t2661, t5610, t9775, t1889, t9779, t828, t9954, t1398, t1868);
        let (t13789, t13790, t13797, t13798, t13801, t13804) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1094(t3935, t828, t1882, t4003, t3957, t5690, t1873, t9741, t5651, t808, t9736, t241, t820, t9991);
    (t13778, t13779, t13781, t13783, t13784, t13789, t13790, t13797, t13798, t13801, t13804)
}
