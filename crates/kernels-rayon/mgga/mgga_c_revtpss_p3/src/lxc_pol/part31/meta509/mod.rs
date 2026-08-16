//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1839;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1840;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1841;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1842;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1843;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1844;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta509(t14587: f64, t27357: f64, t25383: f64, t25388: f64, t25391: f64, t25400: f64, t25406: f64, t25414: f64, t25424: f64, t25432: f64, t27335: f64, t27338: f64, t27342: f64, t27344: f64, t27350: f64, t27353: f64, t27354: f64, t7083: f64, t7766: f64, t7770: f64, t27272: f64, t27297: f64, t27329: f64, t892: f64, t2411: f64, t7782: f64, t1583: f64, t775: f64, t25207: f64, t198: f64, t1993: f64, t11064: f64, t30: f64, t890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27358, t27361) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1839(t14587, t27357, t25383, t25388, t25391, t25400, t25406, t25414, t25424, t25432, t27335, t27338, t27342, t27344, t27350, t27353, t27354, t7083, t7766, t7770);
        let (t27363, t27364) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1840(t27272, t27297, t27329, t27361, t892);
        let t27368 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1841(t2411, t7782);
        let t27375 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1842(t1583, t775);
        let (t27376, t27382) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1843(t25207, t27375, t198, t1993);
        let t27383 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1844(t11064, t30);
        let t27384 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1845(t1583, t890);
    (t27358, t27363, t27364, t27368, t27375, t27376, t27382, t27383, t27384)
}
