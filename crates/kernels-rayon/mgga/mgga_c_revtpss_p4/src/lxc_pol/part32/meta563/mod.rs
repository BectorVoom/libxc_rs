//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1884;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1885;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta563(t14688: f64, t92955: f64, t4452: f64, t92951: f64, t14719: f64, t25227: f64, t2661: f64, t14723: f64, t25266: f64, t4426: f64, t1561: f64, t93048: f64, t14741: f64, t1945: f64, t807: f64, t10886: f64, t4416: f64, t7028: f64, t1549: f64, t92968: f64, t93001: f64, t10778: f64, t1941: f64, t93016: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99021, t99023, t99026, t99029, t99033, t99035) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1884(t14688, t92955, t4452, t92951, t14719, t25227, t2661, t14723, t25266, t4426, t1561, t93048);
        let (t99041, t99044, t99050, t99058, t99062, t99065) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1885(t14741, t1945, t807, t10886, t4416, t7028, t1549, t92968, t93001, t10778, t1941, t93016);
    (t99021, t99023, t99026, t99029, t99033, t99035, t99041, t99044, t99050, t99058, t99062, t99065)
}
