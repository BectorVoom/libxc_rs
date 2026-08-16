//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1083/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1083(t11481: f64, t11484: f64, t11488: f64, t11491: f64, t11494: f64, t11499: f64, t11503: f64, t11507: f64, t11511: f64, t11513: f64, t11516: f64, t11520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39150 = t11481 / 2.0_f64;
    let t39151 = t11484 / 2.0_f64;
    let t39152 = 15.0_f64 / 8.0_f64 * t11488;
    let t39153 = 3.0_f64 / 2.0_f64 * t11491;
    let t39154 = t11494 / 2.0_f64;
    let t39155 = 3.0_f64 / 2.0_f64 * t11499;
    let t39156 = 3.0_f64 / 2.0_f64 * t11503;
    let t39157 = 3.0_f64 / 2.0_f64 * t11507;
    let t39159 = 3.0_f64 * t11511;
    let t39160 = 2.0_f64 * t11513;
    let t39161 = t11516 / 2.0_f64;
    let t39162 = 15.0_f64 / 8.0_f64 * t11520;
    (t39150, t39151, t39152, t39153, t39154, t39155, t39156, t39157, t39159, t39160, t39161, t39162)
}
