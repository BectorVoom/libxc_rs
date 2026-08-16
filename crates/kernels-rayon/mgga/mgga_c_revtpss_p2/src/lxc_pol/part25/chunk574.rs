//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 574/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk574(t1020: f64, t1062: f64, t1053: f64, t1021: f64, t1058: f64, t225: f64, t3043: f64) -> (f64, f64, f64, f64) {
    let t3188 = t1020 * t1062;
    let t3191 = t1020 * t1053;
    let t3194 = t1021 * t1058;
    let t3196 = t3043 * t225;
    (t3188, t3191, t3194, t3196)
}
