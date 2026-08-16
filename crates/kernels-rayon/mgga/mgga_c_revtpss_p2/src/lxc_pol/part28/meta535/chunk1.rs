//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1979/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1979(t5627: f64, t8996: f64, t28167: f64, t1310: f64, t1453: f64, t28050: f64, t28053: f64, t28058: f64, t28060: f64, t28062: f64, t28065: f64, t28069: f64, t28160: f64, t28165: f64, t4248: f64, t508: f64, t649: f64, t651: f64, t7007: f64, t7725: f64, t7883: f64, t7894: f64) -> (f64, f64) {
    let t28168 = t8996 * t5627;
    let t28170 = 6.0_f64 * t28167 * t28168;
    let t28171 = -t1310 * t7725 + t1453 * t7894 - 2.0_f64 * t28050 * t651 - 2.0_f64 * t28053 * t651 - t28160 * t508 - 2.0_f64 * t4248 * t7007 - t649 * t7883 - t28058 - t28060 - t28062 - t28065 - t28069 + t28165 + t28170;
    (t28168, t28171)
}
