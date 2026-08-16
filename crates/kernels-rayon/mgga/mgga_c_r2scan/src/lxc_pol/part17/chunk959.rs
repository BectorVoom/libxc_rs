//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 959/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk959(t2867: f64, t792: f64, t158: f64, t955: f64, t874: f64, t3446: f64, t3447: f64, t122: f64, t3434: f64, t3437: f64, t1103: f64, t2461: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11559 = t2867 * t792;
    let t11563 = t158 * t955;
    let t11564 = t11563 * t874;
    let t11566 = t3446 * t3447 * t11564;
    let t11568 = t11563 * t122;
    let t11570 = t3434 * t3437 * t11568;
    let t11572 = t1103 * t2461;
    (t11559, t11563, t11564, t11566, t11568, t11570, t11572)
}
