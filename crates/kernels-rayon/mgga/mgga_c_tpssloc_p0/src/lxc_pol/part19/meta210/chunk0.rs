//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 889/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk889(t10481: f64, t10482: f64, t1021: f64, t248: f64, t2776: f64, t3051: f64, t1041: f64, t10316: f64, t1044: f64, t3103: f64, t3109: f64, t10309: f64, t3062: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10483 = t10481 * t10482;
    let t10485 = t248 * t1021 * t10483;
    let t10489 = t248 * t3051 * t2776;
    let t10490 = t1041 * t10489;
    let t10493 = t248 * t1044 * t10316;
    let t10496 = t3109 * t3103;
    let t10501 = t248 * t3062 * t10309;
    (t10483, t10485, t10489, t10490, t10493, t10496, t10501)
}
