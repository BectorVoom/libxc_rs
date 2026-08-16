//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta81 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk475;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk476;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk477;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk478;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta81(t1600: f64, t916: f64, t923: f64, t1592: f64, t930: f64, t141: f64, t1594: f64, t921: f64, t929: f64, t935: f64, t915: f64, t939: f64, t948: f64, t951: f64, t954: f64, t958: f64, t324: f64, t967: f64, t970: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1601, t1604, t1606, t1607, t1609, t1610) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk475(t1600, t916, t923, t1592, t930, t141, t1594, t921, t929, t935);
        let (t1612, t1614, t1621, t1622) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk476(t1610, t915, t1594, t939, t1601, t1604, t1607, t948, t951, t954);
        let t1626 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk477(t1594, t958);
        let (t1627, t1633) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk478(t1626, t324, t1594, t1601, t1604, t1607, t967, t970);
        let t1634 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk479(t1633, t973);
    (t1606, t1609, t1610, t1612, t1614, t1621, t1622, t1626, t1627, t1633, t1634)
}
