//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2251/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2251(t16123: f64, t2002: f64, t559: f64, t80920: f64, t80922: f64, t80943: f64, t80957: f64, t80959: f64, t80971: f64, t80989: f64, t80992: f64, t80998: f64, t81007: f64, t91394: f64, t91398: f64, t91400: f64, t91403: f64, t91404: f64, t91406: f64, t91413: f64) -> f64 {
    let t91416 = t16123 * t2002 * t559;
    let t91418 = -119.0_f64 / 6912.0_f64 * t91394 + 0.14130464632949136799e-2_f64 * t80920 + 0.14130464632949136799e-2_f64 * t80922 - 35.0_f64 / 216.0_f64 * t91398 - 0.67826230238155856634e-1_f64 * t91400 + t91403 + 0.16956557559538964158e-1_f64 * t91404 - t91406 - 0.28260929265898273598e-2_f64 * t80943 - t80957 - 0.16956557559538964159e-1_f64 * t80959 + t80971 + 7.0_f64 / 2304.0_f64 * t80989 + 7.0_f64 / 1152.0_f64 * t80992 - 7.0_f64 / 1152.0_f64 * t80998 + 7.0_f64 / 2304.0_f64 * t81007 + t91413 / 192.0_f64 + t91416 / 1536.0_f64;
    t91418
}
