//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1025;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta275(t2453: f64, t4100: f64, t1398: f64, t281: f64, t543: f64, t68: f64, t10115: f64, t562: f64, t2435: f64, t3903: f64, t1445: f64, t3895: f64, t2439: f64, t1420: f64, t3908: f64, t1426: f64, t786: f64, t64: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10139, t10143, t10157, t10160, t10162) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1025(t2453, t4100, t1398, t281, t543, t68, t10115, t562, t2435, t3903, t1445, t3895);
        let (t10163, t10166, t10175, t10199) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1026(t10162, t2439, t1420, t2453, t3908, t1426, t786, t64, t843);
    (t10139, t10143, t10157, t10160, t10163, t10166, t10175, t10199)
}
