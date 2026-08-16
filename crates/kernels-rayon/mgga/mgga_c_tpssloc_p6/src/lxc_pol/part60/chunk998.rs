//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 998/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk998(t23035: f64, t28298: f64, t31366: f64, t121401: f64, t6552: f64, t7479: f64, t121399: f64, t126286: f64, t126290: f64, t126291: f64, t126349: f64, t126352: f64, t126353: f64, t126358: f64, t17090: f64, t2053: f64, t25188: f64, t2718: f64, t28431: f64, t29080: f64, t33443: f64, t33452: f64, t4147: f64, t4268: f64, t6627: f64, t7830: f64, t855: f64, t8553: f64) -> f64 {
    let t127847 = t23035 * t31366 * t28298;
    let t127852 = t6552 * t121401 * t7479;
    let t127858 = -t126286 + t126290 + t126291 + 0.82246703342411321824e-2_f64 * t121399 + 4.0_f64 * t25188 * t7830 + 2.0_f64 * t855 * t2718 * t2053 * t28431 + 4.0_f64 * t4147 * t33443 + 4.0_f64 * t4268 * t33443 + 0.49348022005446793095e-1_f64 * t127847 - t126349 - t126352 - t126353 + 4.0_f64 * t4268 * t33452 + t126358 - 0.3289868133696452873e-1_f64 * t127852 + 2.0_f64 * t17090 * t8553 + 4.0_f64 * t6627 * t29080;
    t127858
}
