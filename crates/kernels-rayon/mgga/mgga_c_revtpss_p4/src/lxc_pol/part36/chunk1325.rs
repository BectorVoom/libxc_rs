//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1325/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1325(t1936: f64, t75941: f64, t1518: f64, t5876: f64, t18245: f64, t7741: f64, t1501: f64, t5920: f64, t30138: f64, t30004: f64, t4248: f64, t22633: f64, t93: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t114372 = 2.0_f64 * t75941 * t1936;
    let t114373 = t5876 * t1518;
    let t114375 = 6.0_f64 * t114373 * t1936;
    let t114377 = 6.0_f64 * t18245 * t7741;
    let t114378 = t1501 * t5920;
    let t114380 = 6.0_f64 * t114378 * t1936;
    let t114382 = 12.0_f64 * t30138 * t7741;
    let t114384 = 6.0_f64 * t4248 * t30004;
    let t114385 = t93 * t22633;
    (t114372, t114373, t114375, t114377, t114378, t114380, t114382, t114384, t114385)
}
