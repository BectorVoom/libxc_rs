//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2034/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2034(t3058: f64, t8521: f64, t7135: f64, t989: f64, t25625: f64, t7166: f64, t11213: f64, t1976: f64, t11711: f64, t25517: f64, t11865: f64, t25516: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93502 = t3058 * t8521;
    let t93509 = t989 * t7135;
    let t93521 = t25625 * t7166;
    let t93528 = t11213 * t1976;
    let t93541 = t25517 * t11711;
    let t93543 = t11865 * t25516;
    (t93502, t93509, t93521, t93528, t93541, t93543)
}
