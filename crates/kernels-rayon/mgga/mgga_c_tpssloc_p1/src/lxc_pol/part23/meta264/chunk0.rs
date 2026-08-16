//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 930/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk930(t20234: f64, t9287: f64, t3981: f64, t5398: f64, t20217: f64, t43: f64, t48: f64, t481: f64, t9300: f64, t3990: f64, t55: f64, t1420: f64, t1423: f64, t39: f64, t51: f64, t5416: f64, t5421: f64, t5424: f64, t56: f64, t9311: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20235 = t9287 * t20234;
    let t20238 = t3981 * t5398;
    let t20241 = t43 * t20217;
    let t20245 = 1.0_f64 / t48 / t481;
    let t20246 = sigma2 * t20245;
    let t20255 = t9300 * t20234;
    let t20258 = t3990 * t5398;
    let t20261 = t55 * t20217;
    let t20264 = -5.0_f64 / 108.0_f64 * t39 * t20235 + 5.0_f64 / 6.0_f64 * t39 * t20238 + 5.0_f64 / 6.0_f64 * t39 * t20241 - 1232.0_f64 / 27.0_f64 * t20246 * t56 - 220.0_f64 / 9.0_f64 * t5416 * t1423 - 20.0_f64 / 9.0_f64 * t1420 * t5421 + 20.0_f64 / 3.0_f64 * t1420 * t5424 + 5.0_f64 / 108.0_f64 * t51 * t20255 + 5.0_f64 / 6.0_f64 * t51 * t20258 - 5.0_f64 / 6.0_f64 * t51 * t20261 + t9311;
    (t20245, t20246, t20255, t20258, t20261, t20264)
}
