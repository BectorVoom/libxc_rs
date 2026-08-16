//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 218/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk218(t1176: f64, t974: f64, t1089: f64, t461: f64, t1169: f64, t221: f64, t456: f64, t1009: f64, t466: f64, t1011: f64, t476: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1177 = t974 * t1176;
    let t1178 = t461 * t1089;
    let t1193 = t221 * t1169;
    let t1195 = t456 * t1193 / 288.0_f64;
    let t1196 = t1176 * t1089;
    let t1206 = t466 * t1009;
    let t1207 = t1206 * t1011;
    let t1208 = t476 * t476;
    let t1209 = 1.0_f64 / t1208;
    let t1210 = t1209 * t478;
    (t1177, t1178, t1193, t1195, t1196, t1206, t1207, t1208, t1209, t1210)
}
