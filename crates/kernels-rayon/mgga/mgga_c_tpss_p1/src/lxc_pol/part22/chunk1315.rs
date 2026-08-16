//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1315/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1315(t136: f64, t1693: f64, t799: f64, t10672: f64, t215: f64, t1395: f64, t2161: f64, t226: f64, t19766: f64, t5567: f64, t36098: f64, t1379: f64, t2407: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63993 = t1693 * t799 * t136;
    let t63995 = t63993 * t215 * t10672;
    let t64007 = t1395 * t2161;
    let t64008 = t64007 * t226;
    let t64034 = t5567 * t19766;
    let t64039 = t36098 * t226;
    let t64042 = t1379 * t2407;
    (t63995, t64007, t64008, t64034, t64039, t64042)
}
