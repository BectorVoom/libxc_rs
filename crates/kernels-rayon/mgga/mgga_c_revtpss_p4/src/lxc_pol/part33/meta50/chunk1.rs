//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 330/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk330(t964: f64, t972: f64, t973: f64, t981: f64, t902: f64, t908: f64, t341: f64) -> (f64, f64, f64, f64, f64) {
    let t983 = t964 * t972 * t973;
    let t985 = 0.5848223622634646207e0_f64 * t981 * t983;
    let t986 = 0.83333333333333333333e-2_f64 * t902;
    let t988 = -t986 - 0.83333333333333333333e-2_f64 * t908;
    let t989 = t988 * t341;
    (t983, t985, t986, t988, t989)
}
