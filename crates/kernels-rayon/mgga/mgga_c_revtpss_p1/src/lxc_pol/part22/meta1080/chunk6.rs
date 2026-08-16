//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3888/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3888(t22294: f64, t48823: f64, t9816: f64, t1398: f64, t6843: f64, t22245: f64, t808: f64, t9736: f64, t22236: f64, t6884: f64, t9741: f64, t13789: f64, t3934: f64, t3938: f64, t47337: f64, t47338: f64, t49126: f64, t49128: f64, t49134: f64, t49139: f64, t49144: f64) -> (f64, f64) {
    let t74698 = t9816 * t48823 * t22294;
    let t74700 = t6843 * t1398;
    let t74711 = t9736 * t808 * t22245;
    let t74714 = t9736 * t808 * t22236;
    let t74717 = t9741 * t6884;
    let t74719 = -0.10164000561857065645e-2_f64 * t74698 + 0.17149607247227894789e-2_f64 * t3934 * t13789 * t74700 * t3938 - 35.0_f64 / 54.0_f64 * t49126 + 7.0_f64 / 6.0_f64 * t49128 + 7.0_f64 / 72.0_f64 * t49134 + 0.22866142996303859718e-3_f64 * t49139 + 0.14291339372689912324e-4_f64 * t49144 + 0.10164000561857065645e-4_f64 * t74711 - 0.50820002809285328225e-4_f64 * t74714 + t47337 - 35.0_f64 / 216.0_f64 * t47338 - 35.0_f64 / 216.0_f64 * t74717;
    (t74700, t74719)
}
