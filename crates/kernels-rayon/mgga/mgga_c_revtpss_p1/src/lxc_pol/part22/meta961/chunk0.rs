//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3223/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3223(t39766: f64, t49926: f64, t49929: f64, t1544: f64, t2408: f64, t49940: f64, t18569: f64, t2398: f64, t39774: f64, t14397: f64, t14436: f64, t18875: f64, t2403: f64, t39760: f64, t39764: f64, t39770: f64, t39773: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61149 = 4.0_f64 * t39766;
    let t61150 = 0.43374325201206959368e-1_f64 * t49926;
    let t61151 = 0.43374325201206959368e-1_f64 * t49929;
    let t61155 = t1544 * t2408;
    let t61159 = 0.70178683471615754484e1_f64 * t49940;
    let t61161 = 8.0_f64 * t2398 * t18569;
    let t61162 = 0.5848223622634646207e0_f64 * t39774;
    let t61163 = -12.0_f64 * t14397 * t18875 * t2403 + 12.0_f64 * t14436 * t2403 * t61155 + t39760 - t39764 + t39770 + t39773 + t61149 - t61150 + t61151 + t61159 + t61161 - t61162;
    (t61149, t61150, t61151, t61159, t61161, t61162, t61163)
}
