//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1227/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1227(t12541: f64, t12543: f64, t1396: f64, t1398: f64, t1404: f64, t16513: f64, t16515: f64, t16548: f64, t1852: f64, t1858: f64, t20149: f64, t20152: f64, t20158: f64, t20186: f64, t5364: f64, t5381: f64, t580: f64, t6471: f64, t6483: f64) -> f64 {
    let tv3rho32 = t1396 * t6483 + t1398 * t20186 + t1404 * t6471 + 2.0_f64 * t1852 * t5381 + 2.0_f64 * t1858 * t5364 + t20149 * t580 + t12541 + t12543 + t16513 + t16515 + t16548 + 2.0_f64 * t20152 + t20158;
    tv3rho32
}
