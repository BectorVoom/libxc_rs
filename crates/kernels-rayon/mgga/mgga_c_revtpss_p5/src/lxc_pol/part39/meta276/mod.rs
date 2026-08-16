//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1014;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta276(t2735: f64, t4086: f64, t3994: f64, t808: f64, t521: f64, t9342: f64, t14: f64, t588: f64, t2496: f64, t4038: f64, t123: f64, t1330: f64, t2630: f64, t2516: f64, t676: f64, t3869: f64, t3926: f64, t3930: f64, t221: f64, t4019: f64, t4057: f64, t4018: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9845, t9847, t9854, t9856, t9858, t9860) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1014(t2735, t4086, t3994, t808, t521, t9342, t14, t588, t2496, t4038, t123, t1330);
        let (t9861, t9863, t9865, t9866, t9868, t9896, t9906) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1015(t2630, t9860, t2516, t676, t3869, t2496, t3926, t3930, t221, t4019, t4057, t4018);
    (t9845, t9847, t9854, t9856, t9858, t9861, t9863, t9865, t9866, t9868, t9896, t9906)
}
