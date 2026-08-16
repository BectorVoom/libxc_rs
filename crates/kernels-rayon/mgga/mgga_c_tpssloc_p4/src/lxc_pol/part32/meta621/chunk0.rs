//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2026/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2026(t27551: f64, t7327: f64, t135: f64, t24847: f64, t7284: f64, t1090: f64, t24821: f64, t1089: f64, t1235: f64, t11708: f64, t24728: f64, t11713: f64, t11715: f64, t11717: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86077 = t7327 * t27551;
    let t86094 = t24847 * t135 * t7284;
    let t86102 = t24821 * t1090;
    let t86116 = t7327 * t1235 * t1089;
    let t86140 = t11708 * t24728;
    let t86146 = t11713 * t11715 * sigma2 * t11717;
    (t86077, t86094, t86102, t86116, t86140, t86146)
}
