//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1289/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1289(t93484: f64, t994: f64, t1071: f64, t7150: f64, t8521: f64, t359: f64, t42066: f64, t1043: f64, t7135: f64, t73: f64, t3143: f64, t36870: f64) -> (f64, f64, f64, f64, f64) {
    let t93959 = t994 * t93484;
    let t93962 = t7150 * t1071;
    let t93963 = t93962 * t8521;
    let t93968 = t42066 * t359;
    let t93974 = t7135 * t1043 * t73;
    let t93982 = t36870 * t3143;
    (t93959, t93963, t93968, t93974, t93982)
}
