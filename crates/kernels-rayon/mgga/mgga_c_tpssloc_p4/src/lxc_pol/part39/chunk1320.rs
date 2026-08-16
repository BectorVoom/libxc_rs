//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1320/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1320(t2281: f64, t8266: f64, t103: f64, t1453: f64, t110333: f64, t110334: f64, t110336: f64, t110338: f64, t110340: f64, t111101: f64, t111104: f64, t111109: f64, t111111: f64, t111121: f64, t111125: f64, t111127: f64, t12808: f64, t2195: f64, t2332: f64, t2350: f64, t2354: f64, t2358: f64, t2585: f64, t29903: f64, t30056: f64, t30293: f64, t30297: f64, t35656: f64, t35663: f64, t4059: f64, t8128: f64, t8137: f64, t8180: f64) -> f64 {
    let t111129 = t2281 * t8266;
    let t111134 = t103 * t1453;
    let t111141 = 44.0_f64 / 9.0_f64 * t110334 - 110.0_f64 / 27.0_f64 * t110336 - 2.0_f64 / 3.0_f64 * t110338 + 5.0_f64 / 9.0_f64 * t110340 + t110333 + 22.0_f64 / 9.0_f64 * t111101 - t111104 + t8128 * t8180 * t12808 / 4.0_f64 + t111109 - t111111 - 5.0_f64 / 12.0_f64 * t8128 * t30293 * t2358 + 25.0_f64 / 72.0_f64 * t8137 * t30297 * t2354 + 5.0_f64 / 4.0_f64 * t29903 * t30293 * t2332 + 25.0_f64 / 108.0_f64 * t8137 * t111121 * t2350 - 55.0_f64 / 27.0_f64 * t111125 - 125.0_f64 / 72.0_f64 * t111127 + 55.0_f64 / 27.0_f64 * t111129 + 5.0_f64 / 24.0_f64 * t2585 * t2195 * t103 - 5.0_f64 / 2.0_f64 * t35656 * t111134 * t30056 + 5.0_f64 / 9.0_f64 * t35663 * t4059 * t30056;
    t111141
}
