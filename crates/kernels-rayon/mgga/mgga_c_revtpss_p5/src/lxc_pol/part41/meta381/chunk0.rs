//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1259/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1259(t15421: f64, t4636: f64, t6110: f64, t934: f64, t2924: f64, t1610: f64, t4631: f64, t2874: f64, t6145: f64, t11299: f64, t6142: f64, t2926: f64, t6141: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19317 = 0.32163958997385070134e2_f64 * t15421 * t4636;
    let t19318 = t6110 * t934;
    let t19320 = 6.0_f64 * t2924 * t19318;
    let t19321 = t1610 * t4631;
    let t19323 = 4.0_f64 * t2874 * t19321;
    let t19324 = t6145 * t934;
    let t19326 = 0.96491876992155210402e2_f64 * t11299 * t19324;
    let t19327 = t6142 * t934;
    let t19329 = 2.0_f64 * t2874 * t19327;
    let t19330 = t6141 * t2926;
    (t19317, t19320, t19323, t19326, t19329, t19330)
}
