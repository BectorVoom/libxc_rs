//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 915/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk915(t1616: f64, t17712: f64, t4582: f64, t1409: f64, t5398: f64, t4588: f64, t10970: f64, t21130: f64, t248: f64, t5681: f64, t3071: f64, t1539: f64, t5873: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21502 = t17712 * t1616;
    let t21503 = t4582 * t21502;
    let t21510 = t5398 * t1409;
    let t21511 = t4588 * t21510;
    let t21512 = t4582 * t21511;
    let t21516 = t248 * t10970 * t21130;
    let t21519 = t5681 * t1616;
    let t21520 = t3071 * t21519;
    let t21525 = t5873 * t1539;
    (t21503, t21510, t21512, t21516, t21520, t21525)
}
