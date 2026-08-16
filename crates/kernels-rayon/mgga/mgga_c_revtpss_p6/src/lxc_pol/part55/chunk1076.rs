//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1076/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1076(t5: f64, t33281: f64, t8737: f64, t32795: f64, t32798: f64, t32802: f64, t32806: f64, t33265: f64, t33270: f64, t33277: f64, t8882: f64, t117: f64, t116: f64, t8885: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t33283 = 5.0_f64 / 27.0_f64 * t8737 * t33281;
    let t33285 = piecewise3(t8, 0.0_f64, -5.0_f64 / 72.0_f64 * t32795 * t8882 + 5.0_f64 / 12.0_f64 * t32798 * t33265 + 5.0_f64 / 18.0_f64 * t32802 * t33270 - 5.0_f64 / 72.0_f64 * t32806 * t8882 - 5.0_f64 / 36.0_f64 * t8737 * t33277 + t33283);
    let t33286 = t33285 * t117;
    let t33287 = t8885 * t116;
    (t33283, t33285, t33286, t33287)
}
