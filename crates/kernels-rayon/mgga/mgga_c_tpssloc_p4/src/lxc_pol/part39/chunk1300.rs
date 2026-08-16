//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1300/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1300(t109: f64, t2332: f64, t8180: f64, t662: f64, t666: f64, t8184: f64, t2358: f64, t2349: f64, t99: f64, t2350: f64, t2354: f64, t29903: f64, t30048: f64, t30049: f64, t30051: f64, t8128: f64, t8137: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t30053 = t8180 * t2332;
    let t30056 = t666 * t662;
    let t30057 = t8184 * t30056;
    let t30060 = t8180 * t2358;
    let t30063 = t99 * t2349;
    let t30064 = t30063 * t2350;
    let t30067 = t8184 * t2354;
    let t30071 = piecewise3(t110, 0.0_f64, -t30048 - 4.0_f64 / 3.0_f64 * t30049 + 10.0_f64 / 9.0_f64 * t30051 - 3.0_f64 / 4.0_f64 * t29903 * t30053 + 5.0_f64 / 6.0_f64 * t8128 * t30057 + t8128 * t30060 / 4.0_f64 - 5.0_f64 / 36.0_f64 * t8137 * t30064 - 5.0_f64 / 24.0_f64 * t8137 * t30067);
    (t30053, t30056, t30057, t30060, t30063, t30064, t30067, t30071)
}
