//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1096/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1096(t3270: f64, t38364: f64, t11477: f64, t11481: f64, t11484: f64, t11488: f64, t11491: f64, t11494: f64, t11499: f64, t11503: f64, t11507: f64, t10614: f64, t10618: f64, t10621: f64, t10625: f64, t10629: f64, t10633: f64) -> (f64, f64) {
    let t38365 = t3270 * t38364;
    let t39149 = 3.0_f64 / 2.0_f64 * t11477;
    let t39150 = t11481 / 2.0_f64;
    let t39151 = t11484 / 2.0_f64;
    let t39152 = 15.0_f64 / 8.0_f64 * t11488;
    let t39153 = 3.0_f64 / 2.0_f64 * t11491;
    let t39154 = t11494 / 2.0_f64;
    let t39155 = 3.0_f64 / 2.0_f64 * t11499;
    let t39156 = 3.0_f64 / 2.0_f64 * t11503;
    let t39157 = 3.0_f64 / 2.0_f64 * t11507;
    let t39158 = t10614 - t39149 - t39150 + t39151 - t39152 + t39153 - t10618 + t10621 - t10625 + t10629 + t10633 - t39154 - t39155 + t39156 + t39157;
    (t38365, t39158)
}
