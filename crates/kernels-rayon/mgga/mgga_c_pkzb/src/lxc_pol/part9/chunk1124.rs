//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1124/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1124(t444: f64, t4803: f64, t1424: f64, t434: f64, t4794: f64, t7: f64, t12584: f64, t1431: f64, t1425: f64, t1430: f64, t15: f64, t82: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19390 = t4803 * t444;
    let t19393 = t434 * t1424;
    let t19396 = t7 * t4794;
    let t19397 = t12584 * t1431;
    let t19400 = t1430 * t1425;
    let t19403 = t1430 * t1431;
    let t19410 = t15 * t82;
    (t19390, t19393, t19396, t19397, t19400, t19403, t19410)
}
