//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1269/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1269(t12379: f64, t6945: f64, t22765: f64, t3853: f64, t80943: f64, t80947: f64, t80950: f64, t80957: f64, t80959: f64, t80963: f64, t80971: f64, t80974: f64, t80978: f64, t80982: f64, t80985: f64, t80987: f64, t80989: f64, t80992: f64, t80994: f64, t80998: f64, t81001: f64, t81003: f64) -> f64 {
    let t81005 = t6945 * t12379;
    let t81007 = t22765 * t3853;
    let t81009 = -0.84782787797694820794e-2_f64 * t80943 + 0.36335480484726351768e-2_f64 * t80947 - 0.12111826828242117256e-2_f64 * t80950 - t80957 - 0.50869672678616892476e-1_f64 * t80959 - 0.25434836339308446237e-1_f64 * t80963 + t80971 - 0.72670960969452703536e-2_f64 * t80974 + 0.36335480484726351768e-2_f64 * t80978 + 0.36335480484726351768e-2_f64 * t80982 + 0.12111826828242117256e-2_f64 * t80985 - t80987 / 1536.0_f64 + 7.0_f64 / 768.0_f64 * t80989 + 7.0_f64 / 384.0_f64 * t80992 - t80994 / 512.0_f64 - 7.0_f64 / 384.0_f64 * t80998 + t81001 / 256.0_f64 - t81003 / 512.0_f64 - t81005 / 1536.0_f64 + 7.0_f64 / 768.0_f64 * t81007;
    t81009
}
