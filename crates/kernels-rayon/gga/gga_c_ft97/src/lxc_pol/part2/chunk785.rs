//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 785/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk785(t12306: f64, t1882: f64, t3327: f64, t3320: f64, t1017: f64, t1570: f64, t1559: f64, t1969: f64, t446: f64, t1986: f64, t925: f64, t9073: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12307 = t12306 / 27.0_f64;
    let t12308 = t1882 * t3327;
    let t12309 = 2.0_f64 / 27.0_f64 * t12308;
    let t12310 = t1882 * t3320;
    let t12311 = 2.0_f64 / 81.0_f64 * t12310;
    let t12312 = t1017 * t1570;
    let t12313 = t12312 * t1559;
    let t12314 = t1969 * t12313;
    let t12315 = t446 * t12314;
    let t12317 = t925 * t1986;
    let t12318 = t9073 * t12317;
    (t12307, t12308, t12309, t12310, t12311, t12313, t12315, t12317, t12318)
}
