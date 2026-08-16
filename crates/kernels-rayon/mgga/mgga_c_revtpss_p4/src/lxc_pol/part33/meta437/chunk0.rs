//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1578/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1578(t18937: f64, t4919: f64, t18913: f64, t16012: f64, t18904: f64, t18926: f64, t4915: f64, t18930: f64, t1062: f64, t6317: f64, t3154: f64, t4866: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19951 = t4919 * t18937;
    let t19954 = t4919 * t18913;
    let t19957 = t16012 * t18904;
    let t19960 = t4915 * t18926;
    let t19963 = t4915 * t18930;
    let t19968 = t6317 * t1062;
    let t19971 = t3154 * t4866;
    (t19951, t19954, t19957, t19960, t19963, t19968, t19971)
}
