//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1084/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1084(t11524: f64, t11526: f64, t11529: f64, t11533: f64, t11535: f64, t11537: f64, t11538: f64, t11541: f64, t11543: f64, t11546: f64, t11548: f64, t11552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39163 = t11524 / 2.0_f64;
    let t39164 = t11526 / 2.0_f64;
    let t39167 = 5.0_f64 / 8.0_f64 * t11529;
    let t39168 = 5.0_f64 / 8.0_f64 * t11533;
    let t39169 = 2.0_f64 * t11535;
    let t39170 = 2.0_f64 * t11537;
    let t39171 = 2.0_f64 * t11538;
    let t39172 = t11541 / 2.0_f64;
    let t39173 = 2.0_f64 * t11543;
    let t39174 = 5.0_f64 / 8.0_f64 * t11546;
    let t39175 = t11548 / 2.0_f64;
    let t39176 = 3.0_f64 / 2.0_f64 * t11552;
    (t39163, t39164, t39167, t39168, t39169, t39170, t39171, t39172, t39173, t39174, t39175, t39176)
}
