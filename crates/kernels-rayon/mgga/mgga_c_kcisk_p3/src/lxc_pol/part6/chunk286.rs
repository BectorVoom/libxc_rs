//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 286/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk286(t442: f64, t459: f64, t306: f64, t425: f64, t458: f64, t382: f64, t394: f64) -> (f64, f64, f64, f64) {
    let t1423 = t459 * t442;
    let t1428 = t306 * t459;
    let t1433 = t458 * t425;
    let t1450 = t394 * t382;
    (t1423, t1428, t1433, t1450)
}
