//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 490/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk490(t1122: f64, t3634: f64, t247: f64, t1261: f64, t1230: f64, t1260: f64, t371: f64, t482: f64, t676: f64, t481: f64, t1231: f64, t1256: f64) -> (f64, f64, f64, f64) {
    let t3635 = t3634 * t1122;
    let t3636 = t247 * t3635;
    let t3637 = t1261 * t3636;
    let t3647 = t1230 * t1260;
    let t3655 = t371 * t676 * t482;
    let t3657 = 0.47637797908966374413e-4_f64 * t481 * t3655;
    let t3658 = t1231 * t1256;
    (t3637, t3647, t3657, t3658)
}
