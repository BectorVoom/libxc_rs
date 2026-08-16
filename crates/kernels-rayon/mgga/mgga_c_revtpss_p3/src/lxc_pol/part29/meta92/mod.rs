//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk558;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk559;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk560;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk561;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk562;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta92(t1945: f64, t213: f64, t248: f64, t209: f64, t785: f64, t251: f64, t1032: f64, t867: f64, t196: f64, t511: f64, t197: f64, t1941: f64, t533: f64, t816: f64, t546: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1946, t1947, t1954, t1955) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk558(t1945, t213, t248, t209, t785);
        let t1956 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk559(t1955, t251);
        let t1957 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk560(t1032, t867);
        let (t2013, t2014) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk561(t196, t511, t197);
        let (t2016, t2018) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk562(t1941, t533, t816, t546, t64);
    (t1946, t1947, t1954, t1955, t1956, t1957, t2013, t2014, t2016, t2018)
}
