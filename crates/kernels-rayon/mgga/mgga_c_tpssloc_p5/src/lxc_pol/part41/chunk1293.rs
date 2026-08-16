//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1293/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1293(t30395: f64, t576: f64, t2212: f64, t5363: f64, t1395: f64, t8299: f64, t1453: f64, t2: f64, t104: f64, t1419: f64, t110334: f64, t110336: f64, t111056: f64, t111058: f64, t111077: f64, t111079: f64, t19525: f64, t19529: f64, t30175: f64, t30293: f64, t30297: f64, t4067: f64, t666: f64, t8128: f64, t8137: f64, t8180: f64, t8184: f64) -> (f64, f64, f64, f64, f64) {
    let t111308 = 2.0_f64 * t576 * t30395;
    let t111310 = 2.0_f64 * t5363 * t2212;
    let t111312 = 2.0_f64 * t1395 * t8299;
    let t111331 = t1453 * t2;
    let t111711 = t1419 * t104;
    let t111715 = t111056 - t111058 - t111077 + t111079 + 22.0_f64 / 9.0_f64 * t110334 - 55.0_f64 / 27.0_f64 * t110336 - 25.0_f64 / 36.0_f64 * t30175 * t30297 * t2 - 5.0_f64 / 24.0_f64 * t8137 * t8184 * t19525 - 5.0_f64 / 6.0_f64 * t8128 * t30293 * t4067 + t8128 * t8180 * t19529 / 4.0_f64 + 10.0_f64 / 9.0_f64 * t8128 * t111711 * t666;
    (t111308, t111310, t111312, t111331, t111715)
}
