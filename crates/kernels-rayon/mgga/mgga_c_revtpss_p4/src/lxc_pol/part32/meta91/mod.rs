//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk556;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk557;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk558;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk559;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk560;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk561;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk562;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta91(t1955: f64, t251: f64, t1032: f64, t867: f64, t196: f64, t511: f64, t197: f64, t1941: f64, t533: f64, t816: f64, t546: f64, t64: f64, t213: f64, t552: f64, t555: f64, t1426: f64, t68: f64, t72: f64, t1927: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1956 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk556(t1955, t251);
        let t1957 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk557(t1032, t867);
        let (t2013, t2014) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk558(t196, t511, t197);
        let (t2016, t2018) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk559(t1941, t533, t816, t546, t64);
        let (t2019, t2020, t2027) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk560(t2018, t213, t552, t1955, t555);
        let t2028 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk561(t1032, t1426);
        let t2047 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk562(t68, t72);
        let t2048 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk563(t1927, t2047);
    (t1956, t1957, t2013, t2014, t2016, t2018, t2019, t2020, t2027, t2028, t2047, t2048)
}
