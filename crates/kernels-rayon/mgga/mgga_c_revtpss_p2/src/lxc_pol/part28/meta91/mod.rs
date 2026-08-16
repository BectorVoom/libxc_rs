//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk573;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk574;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk575;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk576;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk577;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk578;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk579;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk580;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta91(t43: f64, t49: f64, t68: f64, t72: f64, t76: f64, t84: f64, t5: f64, t1923: f64, t117: f64, t114: f64, t112: f64, t508: f64, t651: f64, t198: f64, t207: f64, t159: f64, t215: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1925, t1926) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk573(t43, t49, t68, t72);
        let t1927 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk574(t76, t84);
        let t1928 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk575(t1926, t1927);
        let t1931 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk576(t5, t1923, t1928);
        let t1932 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk577(t117, t1931);
        let t1936 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk578(t114, t112, t68);
        let t1937 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk579(t1936, t508);
        let (t1939, t1940) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk580(t1937, t651, t198, t207);
        let t1941 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk581(t159, t215);
    (t1925, t1926, t1927, t1928, t1931, t1932, t1936, t1937, t1939, t1940, t1941)
}
