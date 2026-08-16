//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1195/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1195(t80956: f64, t80970: f64, t80943: f64, t80947: f64, t80950: f64, t80959: f64, t80963: f64, t80974: f64, t80978: f64, t80982: f64, t80985: f64, t80987: f64, t80989: f64, t80992: f64, t80994: f64, t80998: f64, t81001: f64, t81003: f64, t81005: f64, t81007: f64) -> f64 {
    let t84555 = 0.13958506597733353653e-1_f64 * t80956;
    let t84558 = 0.87474304870637513515e-3_f64 * t80970;
    let t84572 = -0.16956557559538964158e-1_f64 * t80943 + 0.72670960969452703536e-2_f64 * t80947 - 0.24223653656484234512e-2_f64 * t80950 - t84555 - 0.10173934535723378495e0_f64 * t80959 - 0.50869672678616892475e-1_f64 * t80963 + t84558 - 0.14534192193890540707e-1_f64 * t80974 + 0.72670960969452703536e-2_f64 * t80978 + 0.72670960969452703536e-2_f64 * t80982 + 0.24223653656484234512e-2_f64 * t80985 - t80987 / 768.0_f64 + 7.0_f64 / 384.0_f64 * t80989 + 7.0_f64 / 192.0_f64 * t80992 - t80994 / 256.0_f64 - 7.0_f64 / 192.0_f64 * t80998 + t81001 / 128.0_f64 - t81003 / 256.0_f64 - t81005 / 768.0_f64 + 7.0_f64 / 384.0_f64 * t81007;
    t84572
}
