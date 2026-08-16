//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1300/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1300(t18366: f64, t5791: f64, t18660: f64, t5492: f64, t31455: f64, t5784: f64, t18669: f64, t7682: f64, t5489: f64, t18356: f64, t18670: f64, t1675: f64, t1679: f64, t72: f64, t789: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t62273 = t18366 * t5791;
    let t62275 = t5492 * t18660;
    let t62277 = t31455 * t5784;
    let t62280 = t7682 * t18669;
    let t62281 = t62280 * t5489;
    let t62285 = t18670 * t18356;
    let t62294 = 1232.0_f64 / 81.0_f64 * t1675 * t789 * t72 * t1679;
    (t62273, t62275, t62277, t62280, t62281, t62285, t62294)
}
