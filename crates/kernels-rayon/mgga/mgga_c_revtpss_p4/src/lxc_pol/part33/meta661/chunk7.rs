//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2152/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2152(t105923: f64, t25759: f64, t106596: f64, t107882: f64, t107885: f64, t107892: f64, t107895: f64, t107901: f64, t107908: f64, t1940: f64, t1963: f64, t20256: f64, t2403: f64, t25206: f64, t27770: f64, t27793: f64, t27800: f64, t29939: f64, t29949: f64, t29953: f64, t4541: f64, t7087: f64, t98637: f64) -> f64 {
    let t107919 = t25759 * t105923;
    let t107922 = -3.0_f64 / 2.0_f64 * t25206 * t107882 - 3.0_f64 * t25206 * t107885 + 2.0_f64 * t106596 * t27800 - 3.0_f64 * t98637 * t27793 - 3.0_f64 * t25206 * t107892 - 3.0_f64 * t25206 * t107895 + 3.0_f64 * t2403 * t7087 * t29949 + 3.0_f64 * t2403 * t1963 * t107901 + t1940 * t1963 * t20256 / 2.0_f64 + 3.0_f64 * t25206 * t107908 + 3.0_f64 / 2.0_f64 * t2403 * t7087 * t29953 + 3.0_f64 * t4541 * t7087 * t29939 - 3.0_f64 * t98637 * t27770 - 3.0_f64 / 2.0_f64 * t25206 * t107919;
    t107922
}
