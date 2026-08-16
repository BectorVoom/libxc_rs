//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 713/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk713(t1229: f64, t3247: f64, t3961: f64, t4582: f64, t1734: f64, t486: f64, t1215: f64, t3508: f64, t1216: f64, t3242: f64, t3584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4972 = t1229 * t3247;
    let t4973 = t4972 * t3961;
    let t4974 = t4582 * t4973;
    let t4977 = t486 * t1734;
    let t4978 = t3508 * t1215;
    let t4979 = t4977 * t4978;
    let t4980 = t4582 * t4979;
    let t4983 = t4977 * t1216;
    let t4984 = t4582 * t4983;
    let t4987 = t3584 * t3242;
    let t4988 = t4987 * t3961;
    let t4989 = t4582 * t4988;
    (t4972, t4973, t4974, t4977, t4978, t4979, t4980, t4983, t4984, t4987, t4988, t4989)
}
