//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1113/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1113(t12808: f64, t5330: f64, t3153: f64, t3601: f64, t1284: f64, t3555: f64, t3624: f64, t221: f64, t462: f64, t68: f64, t461: f64, t1209: f64, t3766: f64) -> (f64, f64, f64, f64, f64) {
    let t12809 = t12808 * t5330;
    let t12810 = t3601 * t3153;
    let t12831 = t3555 * t1284;
    let t12832 = t12831 * t3624;
    let t12851 = t221 * t68 * t462;
    let t12853 = 5.0_f64 / 1296.0_f64 * t461 * t12851;
    let t12854 = t1209 * t3766;
    (t12809, t12810, t12832, t12853, t12854)
}
