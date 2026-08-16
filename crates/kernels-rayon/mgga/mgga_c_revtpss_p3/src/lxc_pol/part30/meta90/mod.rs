//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta90 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk570;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk571;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk572;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk573;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk574;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk575;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk576;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk577;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta90(t1916: f64, t1918: f64, t572: f64, t573: f64, t38: f64, t603: f64, t76: f64, t84: f64, t114: f64, t112: f64, t68: f64, t508: f64, t651: f64, t198: f64, t207: f64, t159: f64, t215: f64, t218: f64, t816: f64, t234: f64, t64: f64, t213: f64, t248: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1921, t1923) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk570(t1916, t1918, t572, t573, t38, t603);
        let t1927 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk571(t76, t84);
        let t1936 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk572(t114, t112, t68);
        let t1937 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk573(t1936, t508);
        let (t1939, t1940) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk574(t1937, t651, t198, t207);
        let t1941 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk575(t159, t215);
        let (t1943, t1945) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk576(t1941, t218, t816, t234, t64);
        let (t1946, t1949) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk577(t1945, t213, t248, t1943);
        let t1950 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk578(t1949, t225);
    (t1921, t1923, t1927, t1936, t1937, t1939, t1940, t1941, t1945, t1946, t1949, t1950)
}
