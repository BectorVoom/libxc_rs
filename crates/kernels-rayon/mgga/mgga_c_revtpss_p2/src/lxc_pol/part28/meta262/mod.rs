//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1171;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1172;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1173;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1174;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta262(t343: f64, t613: f64, t136: f64, t1007: f64, t1968: f64, t1967: f64, t800: f64, t1020: f64, t1972: f64, t1024: f64, t1035: f64, t1039: f64, sigma0: f64, t1033: f64, t1052: f64, t1971: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7105, t7106, t7110, t7111) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1171(t343, t613, t136, t1007, t1968, t1967, t800);
        let (t7114, t7117) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1172(t1020, t1972, t1024);
        let (t7120, t7121) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1173(t1035, t1039, sigma0);
        let t7122 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1174(t1033, t7121);
        let t7125 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1175(t1052, t1971);
    (t7105, t7106, t7110, t7111, t7114, t7117, t7120, t7121, t7122, t7125)
}
