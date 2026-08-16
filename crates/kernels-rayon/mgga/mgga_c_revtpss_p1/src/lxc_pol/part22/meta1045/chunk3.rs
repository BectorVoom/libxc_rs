//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3662/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3662(t1168: f64, t1187: f64, t12476: f64, t16948: f64, t16959: f64, t16979: f64, t16989: f64, t435: f64, t57944: f64, t57972: f64, t58237: f64, t58259: f64, t58300: f64, t58345: f64, t58592: f64, t58647: f64, t6535: f64, t68250: f64, t68633: f64, t68636: f64, t68640: f64, t68694: f64, t68711: f64, t68730: f64, t68751: f64, t68754: f64, t68757: f64, t69139: f64, t69153: f64, t69167: f64, t69181: f64) -> f64 {
    let t69192 = 0.14035736694323150897e2_f64 * t58345 * t16979 - 0.77193501593724168323e3_f64 * t57944 * t16959 + 0.41016075432865626631e4_f64 * t58300 * t58647 * t1187 + t68250 + 0.8276162067083744048e4_f64 * t58592 * t57972 * t1168 - t68633 - t68636 - t68640 + t68694 - t68711 - 0.310907e-1_f64 * (t69139 + t69153 + t69167 + t69181) * t435 - 0.4155806185363551302e3_f64 * t58259 * t16989 - t68730 + 24.0_f64 * t58237 * t16948 + 0.5848223622634646207e0_f64 * t12476 * t6535 - t68751 - t68754 + t68757;
    t69192
}
