//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1426/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1426(t1099: f64, t1118: f64, t44021: f64, t44036: f64, t44052: f64, t44067: f64, t3311: f64, t409: f64, t3314: f64, t43970: f64, t11185: f64, t11427: f64) -> (f64, f64, f64) {
    let t44072 = 1.0_f64 * t1099 * (t44021 + t44036 + t44052 + t44067) * t1118;
    let t44073 = t3311 * t3311;
    let t44075 = t409 / t44073;
    let t44076 = t3314 * t3314;
    let t44077 = 1.0_f64 / t44076;
    let t44080 = 0.24955700379505800916e5_f64 * t44075 * t43970 * t44077;
    let t44082 = 24.0_f64 * t11185 * t11427;
    (t44072, t44080, t44082)
}
