//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1614/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1614(t12269: f64, t1261: f64, t12884: f64, t247: f64, t13085: f64, t3647: f64, t12277: f64, t3634: f64, t13089: f64, t12273: f64, t1042: f64, t1122: f64, t12629: f64, t1263: f64, t12926: f64, t13076: f64, t17202: f64, t17344: f64, t2251: f64, t3363: f64, t3568: f64, t3584: f64, t3617: f64, t3618: f64, t3708: f64, t3711: f64, t43767: f64, t43869: f64, t44205: f64, t44362: f64, t44375: f64, t44377: f64, t44378: f64, t5268: f64, t5384: f64) -> f64 {
    let t44403 = t1261 * t247 * t12884 * t12269;
    let t44405 = t3647 * t13085;
    let t44409 = t1261 * t247 * t3634 * t12277;
    let t44411 = t3647 * t13089;
    let t44415 = t1261 * t247 * t3634 * t12273;
    let t44417 = 0.71456696863449561621e-3_f64 * t1261 * t247 * t3618 * t43869 + 0.23289590088828005269e-2_f64 * t1261 * t247 * t44362 * t43767 + 0.28582678745379824648e-2_f64 * t5384 * t1042 * t3617 * t3568 * t3363 - 0.21437009059034868486e-3_f64 * t44375 * t1042 * t44377 * t44378 + 0.17149607247227894789e-2_f64 * t3711 * t1042 * t5268 * t2251 * t3584 + 0.34299214494455789577e-2_f64 * t3711 * t1042 * t17202 * t44205 - 0.34299214494455789578e-2_f64 * t3647 * t12926 + 0.34299214494455789578e-2_f64 * t17344 * t1042 * t1263 * t12629 * t1122 + 0.85748036236139473944e-3_f64 * t3708 * t13076 + 0.38110238327173099531e-2_f64 * t44403 - 0.11433071498151929859e-2_f64 * t44405 - 0.38110238327173099531e-3_f64 * t44409 - 0.22866142996303859718e-2_f64 * t44411 - 0.22866142996303859718e-2_f64 * t44415;
    t44417
}
