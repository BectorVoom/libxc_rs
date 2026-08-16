//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1023/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1023(t1211: f64, t6573: f64, t1774: f64, t1828: f64, t1277: f64, t3579: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64) -> (f64, f64, f64) {
    let t6574 = t1211 * t6573;
    let t6579 = t1774 * t1828;
    let t6580 = t1277 * t6579;
    let t6587 = t3579 - 0.9877777777777777778e-2_f64 * t5044 - 0.9877777777777777778e-2_f64 * t6423 + 0.29633333333333333334e-1_f64 * t6427 + 0.14816666666666666667e-1_f64 * t6431;
    (t6574, t6580, t6587)
}
