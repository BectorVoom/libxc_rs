//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2158/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2158(t107922: f64, t107963: f64, t108001: f64, t108047: f64, t22279: f64, t28167: f64, t8996: f64, t29506: f64, t7313: f64, t1843: f64, t28042: f64, t651: f64) -> (f64, f64, f64, f64) {
    let t108049 = t107922 + t107963 + t108001 + t108047;
    let t108067 = 12.0_f64 * t28167 * t8996 * t22279;
    let t108068 = t29506 * t7313;
    let t108076 = 4.0_f64 * t651 * t1843 * t28042;
    (t108049, t108067, t108068, t108076)
}
