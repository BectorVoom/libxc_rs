//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1364;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1365;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta349(t221: f64, t3979: f64, t5591: f64, t3978: f64, t3989: f64, t5614: f64, t5622: f64, t9765: f64, t1408: f64, t240: f64, t1868: f64, t4010: f64, t1353: f64, t2661: f64, t550: f64, t5658: f64, t543: f64, t3992: f64, t5610: f64, t9775: f64, t1889: f64, t9779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13760, t13762, t13763, t13765, t13767, t13768) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1364(t221, t3979, t5591, t3978, t3989, t5614, t5622, t9765, t1408, t240, t1868, t4010);
        let (t13769, t13772, t13775, t13778, t13779, t13781) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1365(t1353, t13768, t13767, t2661, t550, t5658, t543, t3992, t5610, t9775, t1889, t9779);
    (t13760, t13762, t13763, t13765, t13769, t13772, t13775, t13778, t13779, t13781)
}
