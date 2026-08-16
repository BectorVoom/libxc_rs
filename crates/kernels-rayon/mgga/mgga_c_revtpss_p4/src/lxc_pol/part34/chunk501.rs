//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 501/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk501(t1121: f64, t471: f64, t126: f64, t1263: f64, t371: f64, t482: f64, t676: f64, t481: f64, t225: f64, t3566: f64) -> (f64, f64, f64, f64, f64) {
    let t3628 = t471 * t1121;
    let t3634 = t126 * t1263;
    let t3655 = t371 * t676 * t482;
    let t3657 = 0.47637797908966374413e-4_f64 * t481 * t3655;
    let t3670 = t3566 * t225;
    (t3628, t3634, t3655, t3657, t3670)
}
