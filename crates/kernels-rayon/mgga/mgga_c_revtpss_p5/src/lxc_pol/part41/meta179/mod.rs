//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk745;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta179(t1045: f64, t373: f64, t4866: f64, t1042: f64, t1065: f64, t905: f64, t1469: f64, t999: f64, t1032: f64, t1647: f64, t1040: f64, t1025: f64, t1028: f64, t1041: f64, t1047: f64, t1665: f64, t1671: f64, t3124: f64, t3127: f64, t3194: f64, t3203: f64, t3211: f64, t3216: f64, t3224: f64, t4854: f64, t4858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4868, t4869, t4872, t4873, t4874, t4875, t4878, t4879) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk745(t1045, t373, t4866, t1042, t1065, t905, t1469, t999, t1032, t1647, t1040);
        let t4883 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk746(t1025, t1028, t1041, t1047, t1665, t1671, t3124, t3127, t3194, t3203, t3211, t3216, t3224, t4854, t4858, t4869, t4875, t4879);
    (t4868, t4869, t4872, t4873, t4874, t4875, t4878, t4879, t4883)
}
