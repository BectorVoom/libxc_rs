//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1021;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1022;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta278(t240: f64, t9940: f64, t72: f64, t595: f64, t66: f64, t247: f64, t550: f64, t548: f64, t4010: f64, t245: f64, t3829: f64, t543: f64, t1386: f64, t820: f64, t844: f64, t3940: f64, t221: f64, t3924: f64, t4019: f64, t4018: f64, t3930: f64, t4059: f64, t2482: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9942, t9949, t9953, t9954, t9955, t9956) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1021(t240, t9940, t72, t595, t66, t247, t550, t548, t4010, t245, t3829, t543);
        let (t9962, t9963, t9971, t9973, t9976) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1022(t1386, t820, t844, t3940, t221, t3924, t4019, t4018, t3930, t4059, t2482, t596);
    (t9942, t9949, t9953, t9954, t9955, t9956, t9962, t9963, t9971, t9973, t9976)
}
