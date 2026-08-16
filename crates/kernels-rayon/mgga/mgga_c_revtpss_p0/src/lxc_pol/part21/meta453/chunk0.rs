//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1982/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1982(t45: f64, t11064: f64, t1583: f64, t1469: f64, t2609: f64, t706: f64, t10593: f64, t10597: f64, t4186: f64, t80: f64, t13312: f64, t1490: f64, t2251: f64, t2258: f64, t4328: f64, t606: f64, t766: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t14436 = t1583 * t11064;
    let t14440 = t2609 * t1469;
    let t14441 = t706 * t14440;
    let t14442 = 4.0_f64 * t14441;
    let t14443 = 0.11696447245269292414e1_f64 * t10593;
    let t14444 = 0.34631718211362927518e2_f64 * t10597;
    let t14447 = t80 * t4186;
    let t14455 = piecewise3(t151, 0.0_f64, 8.0_f64 / 27.0_f64 * t1490 * t2251 - 4.0_f64 / 9.0_f64 * t14447 * t606 - 2.0_f64 / 9.0_f64 * t4328 * t2258 + 2.0_f64 / 3.0_f64 * t766 * t13312);
    (t14436, t14440, t14442, t14443, t14444, t14447, t14455)
}
