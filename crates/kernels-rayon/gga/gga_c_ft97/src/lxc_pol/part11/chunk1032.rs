//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1032/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1032(t41534: f64, t41536: f64, t41448: f64, t420: f64, t701: f64, t41499: f64, t41502: f64, t41505: f64, t41508: f64, t41513: f64, t41516: f64, t41519: f64, t41522: f64, t41525: f64, t41528: f64, t41531: f64) -> (f64, f64) {
    let t41537 = t41534 * t41536;
    let t41540 = t701 * t420 * t41537 * t41448;
    let t41542 = -0.68099848938271604939e-1_f64 * t41499 + 0.34049924469135802468e-1_f64 * t41502 + 0.51074886703703703704e-1_f64 * t41505 - 0.51074886703703703704e-1_f64 * t41508 - t41513 + 0.6384360837962962963e-2_f64 * t41516 + 0.94583123525377229081e-2_f64 * t41519 - 0.85124811172839506172e-2_f64 * t41522 - 0.1134997482304526749e-1_f64 * t41525 + 0.26483274587105624143e-1_f64 * t41528 + 0.85124811172839506172e-2_f64 * t41531 + 0.66208186467764060357e-1_f64 * t41540;
    (t41540, t41542)
}
