//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta214(t1592: f64, t4786: f64, t3092: f64, t1058: f64, t1660: f64, t1053: f64, t1659: f64, t225: f64, t4743: f64, t366: f64, t1065: f64, t2857: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t4787, t4788, t4792, t4794, t4797, t4798, t4801) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk861(t1592, t4786, t3092, t1058, t1660, t1053, t1659, t225, t4743, t366, t1065, t2857);
    (t4787, t4788, t4792, t4794, t4797, t4798, t4801)
}
