//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1614/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1614<F: Float>(t12269: F, t1261: F, t12884: F, t247: F, t13085: F, t3647: F, t12277: F, t3634: F, t13089: F, t12273: F, t1042: F, t1122: F, t12629: F, t1263: F, t12926: F, t13076: F, t17202: F, t17344: F, t2251: F, t3363: F, t3568: F, t3584: F, t3617: F, t3618: F, t3708: F, t3711: F, t43767: F, t43869: F, t44205: F, t44362: F, t44375: F, t44377: F, t44378: F, t5268: F, t5384: F) -> F {
    let t44403 = t1261 * t247 * t12884 * t12269;
    let t44405 = t3647 * t13085;
    let t44409 = t1261 * t247 * t3634 * t12277;
    let t44411 = t3647 * t13089;
    let t44415 = t1261 * t247 * t3634 * t12273;
    let t44417 = F::cast_from(0.71456696863449561621e-3_f64) * t1261 * t247 * t3618 * t43869 + F::cast_from(0.23289590088828005269e-2_f64) * t1261 * t247 * t44362 * t43767 + F::cast_from(0.28582678745379824648e-2_f64) * t5384 * t1042 * t3617 * t3568 * t3363 - F::cast_from(0.21437009059034868486e-3_f64) * t44375 * t1042 * t44377 * t44378 + F::cast_from(0.17149607247227894789e-2_f64) * t3711 * t1042 * t5268 * t2251 * t3584 + F::cast_from(0.34299214494455789577e-2_f64) * t3711 * t1042 * t17202 * t44205 - F::cast_from(0.34299214494455789578e-2_f64) * t3647 * t12926 + F::cast_from(0.34299214494455789578e-2_f64) * t17344 * t1042 * t1263 * t12629 * t1122 + F::cast_from(0.85748036236139473944e-3_f64) * t3708 * t13076 + F::cast_from(0.38110238327173099531e-2_f64) * t44403 - F::cast_from(0.11433071498151929859e-2_f64) * t44405 - F::cast_from(0.38110238327173099531e-3_f64) * t44409 - F::cast_from(0.22866142996303859718e-2_f64) * t44411 - F::cast_from(0.22866142996303859718e-2_f64) * t44415;
    t44417
}
