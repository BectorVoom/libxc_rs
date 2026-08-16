//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1313/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1313(t5: f64, t114267: f64, t114292: f64, t114320: f64, t114356: f64, t117: f64, t5883: f64, t7724: f64, t1936: f64, t75941: f64, t1518: f64, t5876: f64, t18245: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t114359 = piecewise3(t8, 0.0_f64, t114267 + t114292 + t114320 + t114356);
    let t114360 = t114359 * t117;
    let t114363 = t7724 * t5883;
    let t114372 = 2.0_f64 * t75941 * t1936;
    let t114373 = t5876 * t1518;
    let t114375 = 6.0_f64 * t114373 * t1936;
    let t114377 = 6.0_f64 * t18245 * t7741;
    (t114360, t114363, t114372, t114373, t114375, t114377)
}
