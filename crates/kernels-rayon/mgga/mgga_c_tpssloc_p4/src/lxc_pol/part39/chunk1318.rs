//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1318/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1318(t110075: f64, t30281: f64, t29895: f64, t30285: f64, t30304: f64, t29900: f64, t30308: f64, t110082: f64, t110290: f64, t110292: f64, t110294: f64, t110314: f64, t110601: f64, t110602: f64, t1449: f64, t1453: f64, t2: f64, t2332: f64, t2350: f64, t2354: f64, t2358: f64, t29903: f64, t30056: f64, t30063: f64, t30175: f64, t30297: f64, t4067: f64, t662: f64, t8128: f64, t8137: f64, t8180: f64, t8184: f64, t86592: f64, t86595: f64, t86598: f64) -> f64 {
    let t111056 = 4.0_f64 * t110075 * t30281;
    let t111058 = 20.0_f64 / 9.0_f64 * t29895 * t30285;
    let t111077 = 20.0_f64 / 9.0_f64 * t29895 * t30304;
    let t111079 = 20.0_f64 / 27.0_f64 * t29900 * t30308;
    let t111096 = 2.0_f64 * t110290 + 10.0_f64 / 27.0_f64 * t110294 + 3.0_f64 * t110082 * t8180 * t86592 + 5.0_f64 / 18.0_f64 * t8128 * t30063 * t1453 * t2350 - 5.0_f64 / 4.0_f64 * t29903 * t8184 * t1449 * t2332 + 5.0_f64 / 108.0_f64 * t8137 * t110314 * t1449 * t2350 + t111056 - t111058 - 3.0_f64 / 2.0_f64 * t29903 * t8180 * t86595 - 3.0_f64 / 4.0_f64 * t29903 * t8180 * t86598 + 5.0_f64 / 6.0_f64 * t8128 * t8184 * t4067 * t662 + 5.0_f64 / 12.0_f64 * t8128 * t8184 * t1453 * t2354 - 25.0_f64 / 18.0_f64 * t8128 * t30297 * t30056 - t111077 + t111079 + 5.0_f64 / 12.0_f64 * t8128 * t8184 * t1449 * t2358 - 5.0_f64 / 6.0_f64 * t110601 * t8184 * t110602 - 5.0_f64 / 36.0_f64 * t8137 * t30063 * t1449 * t2354 + 5.0_f64 / 18.0_f64 * t30175 * t30063 * t2 * t662 - 20.0_f64 / 9.0_f64 * t110292;
    t111096
}
