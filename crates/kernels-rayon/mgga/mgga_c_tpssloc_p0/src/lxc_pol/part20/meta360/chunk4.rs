//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1688/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1688(t12465: f64, t12474: f64, t12476: f64, t12490: f64, t3652: f64, t671: f64, t1266: f64, t2363: f64, t113: f64, t11968: f64, t11972: f64, t1271: f64, t1393: f64, t2312: f64, t2314: f64, t2320: f64, t2323: f64, t2364: f64, t3660: f64, t3929: f64, t4034: f64, t510: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64, t9347: f64, t9348: f64, t9351: f64, t9419: f64) -> (f64, f64, f64, f64) {
    let t12492 = t12465 + t12474 + t12476 + t12490;
    let t12504 = t3652 * t671;
    let t12507 = t1266 * t2363;
    let t12512 = -t113 * t11968 - 2.0_f64 * t11972 * t652 + t12492 * t513 - 6.0_f64 * t12504 * t652 - 6.0_f64 * t12507 * t652 - 3.0_f64 * t1266 * t2312 - 6.0_f64 * t1266 * t2320 + 3.0_f64 * t1271 * t3929 + 3.0_f64 * t1393 * t3660 - 12.0_f64 * t2314 * t2323 - 6.0_f64 * t2314 * t2364 - 6.0_f64 * t2364 * t4034 - 3.0_f64 * t3652 * t650 - t510 * t9347 - 6.0_f64 * t510 * t9351 + t574 * t9419 - 6.0_f64 * t672 * t9348;
    (t12492, t12504, t12507, t12512)
}
