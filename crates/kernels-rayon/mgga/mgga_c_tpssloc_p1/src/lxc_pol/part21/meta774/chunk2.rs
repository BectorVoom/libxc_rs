//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2682/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2682(t157: f64, t56323: f64, t56347: f64, t182: f64, t1390: f64, t20063: f64, t54412: f64, t39491: f64, t12466: f64, t1307: f64, t16148: f64, t3918: f64, t39483: f64, t39490: f64, t39496: f64, t5122: f64, t5126: f64, t56298: f64, t56299: f64, t6330: f64) -> (f64, f64, f64, f64, f64) {
    let t56349 = (t56323 + t56347) * t157;
    let t56351 = 0.19751673498613801407e-1_f64 * t56349 * t182;
    let t56358 = t20063 * t1390;
    let t56362 = 24.0_f64 * t54412;
    let t56363 = 0.11696447245269292414e1_f64 * t39491;
    let t56364 = 6.0_f64 * t12466 * t5126 * t6330 + 6.0_f64 * t1307 * t3918 * t56358 + 24.0_f64 * t16148 * t5122 * t5126 + t39483 - t39490 - t39496 + t56298 + t56299 + t56351 - t56362 + t56363;
    (t56349, t56351, t56362, t56363, t56364)
}
