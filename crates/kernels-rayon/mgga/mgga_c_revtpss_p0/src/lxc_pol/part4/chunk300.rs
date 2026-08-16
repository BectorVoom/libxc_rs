//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 300/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk300(t902: f64, t928: f64, t908: f64, t919: f64, t924: f64, t932: f64) -> (f64, f64, f64) {
    let t967 = 0.301925e0_f64 * t902;
    let t970 = 0.82785e-1_f64 * t928;
    let t972 = 0.258925e1_f64 * t919 - t967 - 0.301925e0_f64 * t908 + 0.16504875e0_f64 * t924 - t970 - 0.82785e-1_f64 * t932;
    (t967, t970, t972)
}
