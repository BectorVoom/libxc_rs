//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta90 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk580;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk581;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk582;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk583;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk584;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk585;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk586;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta90(t1955: f64, t251: f64, t1032: f64, t867: f64, t1949: f64, t233: f64, t1951: f64, t213: f64, t892: f64, t30: f64, t1940: f64, t207: f64, t198: f64, t33: f64, t1312: f64, t1936: f64, t196: f64, t511: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1956 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk580(t1955, t251);
        let t1957 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk581(t1032, t867);
        let t1958 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk582(t1949, t233);
        let t1959 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk583(t1957, t1958);
        let t1962 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk584(t1951, t1956, t1959, t213);
        let t1963 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk585(t1962, t892);
        let (t1966, t1993, t1995, t2002, t2010, t2013) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk586(t1963, t30, t1940, t1962, t207, t198, t892, t33, t1312, t1936, t196, t511);
    (t1956, t1957, t1958, t1959, t1962, t1963, t1966, t1993, t1995, t2002, t2010, t2013)
}
