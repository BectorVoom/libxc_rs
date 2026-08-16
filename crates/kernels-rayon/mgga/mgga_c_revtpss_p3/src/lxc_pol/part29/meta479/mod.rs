//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1755;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1756;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta479(t4345: f64, t7045: f64, t25234: f64, t4349: f64, t25227: f64, t4353: f64, t2661: f64, t1565: f64, t25222: f64, t241: f64, t25260: f64, t820: f64, t4368: f64, t1955: f64, t4469: f64, t1579: f64, t231: f64, t836: f64, t1559: f64, t886: f64, t7057: f64, t1583: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27249, t27251, t27253, t27254, t27256, t27261) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1755(t4345, t7045, t25234, t4349, t25227, t4353, t2661, t1565, t25222, t241, t25260, t820);
        let (t27262, t27275, t27312, t27349, t27353) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1756(t27261, t4368, t1955, t4469, t1579, t231, t836, t1559, t886, t7057);
        let t27375 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1757(t1583, t775);
    (t27249, t27251, t27253, t27254, t27256, t27261, t27262, t27275, t27312, t27349, t27353, t27375)
}
