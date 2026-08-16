//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta91 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk580;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk581;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk582;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk583;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk584;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk585;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk586;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk587;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta91(t1945: f64, t213: f64, t248: f64, t1943: f64, t225: f64, t257: f64, t209: f64, t785: f64, t251: f64, t1032: f64, t867: f64, t233: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1946, t1949) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk580(t1945, t213, t248, t1943);
        let t1950 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk581(t1949, t225);
        let (t1951, t1954, t1955) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk582(t1950, t257, t209, t785);
        let t1956 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk583(t1955, t251);
        let t1957 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk584(t1032, t867);
        let t1958 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk585(t1949, t233);
        let t1959 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk586(t1957, t1958);
        let t1962 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk587(t1951, t1956, t1959, t213);
        let t1963 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk588(t1962, t892);
    (t1946, t1949, t1950, t1951, t1954, t1955, t1956, t1957, t1958, t1959, t1962, t1963)
}
