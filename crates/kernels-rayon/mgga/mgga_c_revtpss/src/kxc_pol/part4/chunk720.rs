//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 720/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk720(t1122: f64, t3634: f64, t247: f64, t1261: f64, t1264: f64, t3372: f64, t3368: f64, t1230: f64, t1260: f64) -> (f64, f64, f64, f64, f64) {
    let t3635 = t3634 * t1122;
    let t3636 = t247 * t3635;
    let t3637 = t1261 * t3636;
    let t3639 = t1264 * t3372;
    let t3640 = t247 * t3639;
    let t3643 = t1264 * t3368;
    let t3644 = t247 * t3643;
    let t3647 = t1230 * t1260;
    (t3636, t3637, t3640, t3644, t3647)
}
