//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1477/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1477(t6243: f64, t1751: f64, t22298: f64, t491: f64, t78757: f64, t6224: f64, t6238: f64, t11914: f64, t11915: f64, t1244: f64, t1246: f64, t15245: f64, t1734: f64, t1755: f64, t1756: f64, t19201: f64, t22243: f64, t22327: f64, t22354: f64, t22355: f64, t22389: f64, t3610: f64, t3612: f64, t3624: f64, t3625: f64, t6218: f64, t6252: f64, t6253: f64, t6257: f64, t65254: f64, t73630: f64) -> (f64, f64, f64, f64, f64) {
    let t79398 = t6243 * t6243;
    let t79410 = t1751 * t22298;
    let t79453 = t491 * t78757;
    let t79461 = t6238 * t6224;
    let t79467 = 6.0_f64 * t11914 * t11915 * t6218 * t6252 + 4.0_f64 * t1244 * t1246 * t1734 * t22327 + 8.0_f64 * t1755 * t22243 * t3610 * t3612 + 4.0_f64 * t11914 * t11915 * t79410 - 12.0_f64 * t22354 * t22389 * t3624 - 3.0_f64 * t3624 * t3625 * t79453 - 6.0_f64 * t3624 * t3625 * t79461 - 12.0_f64 * t15245 * t22355 + 4.0_f64 * t1756 * t73630 + 12.0_f64 * t19201 * t6257 + 12.0_f64 * t6253 * t65254;
    (t79398, t79410, t79453, t79461, t79467)
}
