//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 510/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk510(t225: f64, t3736: f64, t1284: f64, t487: f64, t1209: f64, t3140: f64, t3596: f64, t460: f64, t3303: f64, t3603: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3737 = t225 * t3736;
    let t3754 = t1284 * t487;
    let t3755 = t1209 * t3754;
    let t3766 = t3140 * t3596;
    let t3767 = t460 * t3766;
    let t3769 = t3303 * t3603;
    (t3737, t3754, t3755, t3766, t3767, t3769)
}
