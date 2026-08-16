//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1380/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1380(t5: f64, t67342: f64, t67362: f64, t67387: f64, t67407: f64, t67434: f64, t67462: f64, t67489: f64, t67514: f64, t117: f64, t65440: f64, t65442: f64, t65444: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t67518 = piecewise3(t8, 0.0_f64, t67342 + t67362 + t67387 + t67407 + t67434 + t67462 + t67489 + t67514);
    let t67519 = t67518 * t117;
    let t67531 = 22.0_f64 / 9.0_f64 * t65440;
    let t67532 = 8.0_f64 / 3.0_f64 * t65442;
    let t67533 = 4.0_f64 / 3.0_f64 * t65444;
    (t67519, t67531, t67532, t67533)
}
