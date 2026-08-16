//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 220/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk220(t1193: f64, t456: f64, t1089: f64, t1176: f64, t1009: f64, t466: f64, t1011: f64, t476: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1195 = t456 * t1193 / 288.0_f64;
    let t1196 = t1176 * t1089;
    let t1206 = t466 * t1009;
    let t1207 = t1206 * t1011;
    let t1208 = t476 * t476;
    let t1209 = 1.0_f64 / t1208;
    (t1195, t1196, t1206, t1207, t1208, t1209)
}
