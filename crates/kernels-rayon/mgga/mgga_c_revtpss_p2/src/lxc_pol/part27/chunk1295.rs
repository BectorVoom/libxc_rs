//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1295/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1295(t1459: f64, t26120: f64, t26124: f64, t26127: f64, t1455: f64, t7700: f64, t1464: f64, t7690: f64, t2167: f64, t4168: f64, t27089: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95171 = 18.0_f64 * t1459 * t26120;
    let t95173 = 36.0_f64 * t1459 * t26124;
    let t95175 = 18.0_f64 * t1459 * t26127;
    let t96684 = t1455 * t7700;
    let t96690 = t7690 * t1464;
    let t96692 = t2167 * t4168;
    let t96694 = t27089 * t575;
    (t95171, t95173, t95175, t96684, t96690, t96692, t96694)
}
