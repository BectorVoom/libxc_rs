//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 266/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk266(t1169: f64, t221: f64, t456: f64, t1089: f64, t1176: f64, t607: f64, t974: f64, t1190: f64, t225: f64, t68: f64, t484: f64, t1009: f64, t466: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1193 = t221 * t1169;
    let t1195 = t456 * t1193 / 288.0_f64;
    let t1196 = t1176 * t1089;
    let t1197 = t1196 * t607;
    let t1198 = t974 * t1197;
    let t1201 = t1190 * t225;
    let t1202 = t1201 * t68;
    let t1203 = t1202 * t484;
    let t1206 = t466 * t1009;
    (t1195, t1196, t1197, t1198, t1201, t1202, t1203, t1206)
}
