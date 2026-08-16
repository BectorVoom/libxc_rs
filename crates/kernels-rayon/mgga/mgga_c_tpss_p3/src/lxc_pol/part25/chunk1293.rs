//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1293/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1293(t1656: f64, t18967: f64, t20155: f64, t219: f64, t65551: f64, t65561: f64, t65570: f64, t65592: f64, t65600: f64, t65616: f64, t65628: f64, t65639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t67061 = t18967 * t1656;
    let t67083 = t20155 * t219;
    let t67138 = 7.0_f64 / 576.0_f64 * t65551;
    let t67143 = 7.0_f64 / 144.0_f64 * t65561;
    let t67150 = 7.0_f64 / 36.0_f64 * t65570;
    let t67160 = 7.0_f64 / 288.0_f64 * t65592;
    let t67162 = 7.0_f64 / 12.0_f64 * t65600;
    let t67169 = 35.0_f64 / 144.0_f64 * t65616;
    let t67175 = 7.0_f64 / 576.0_f64 * t65628;
    let t67183 = 7.0_f64 / 144.0_f64 * t65639;
    (t67061, t67083, t67138, t67143, t67150, t67160, t67162, t67169, t67175, t67183)
}
