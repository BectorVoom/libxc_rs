//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1059/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1059(t1215: f64, t3508: f64, t4977: f64, t4582: f64, t1216: f64, t3242: f64, t3584: f64, t3961: f64, t1653: f64, t248: f64, t3521: f64, t1227: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4978 = t3508 * t1215;
    let t4979 = t4977 * t4978;
    let t4980 = t4582 * t4979;
    let t4983 = t4977 * t1216;
    let t4984 = t4582 * t4983;
    let t4987 = t3584 * t3242;
    let t4988 = t4987 * t3961;
    let t4989 = t4582 * t4988;
    let t4993 = t248 * t3521 * t1653;
    let t4994 = t1227 * t4993;
    (t4978, t4979, t4980, t4983, t4984, t4987, t4988, t4989, t4993, t4994)
}
