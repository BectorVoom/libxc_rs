//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 983/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk983(t11563: f64, t122: f64, t3434: f64, t3437: f64, t1103: f64, t2461: f64, t1053: f64, t1102: f64, t10935: f64, t3446: f64, t970: f64, t58: f64, t897: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11568 = t11563 * t122;
    let t11570 = t3434 * t3437 * t11568;
    let t11572 = t1103 * t2461;
    let t11574 = t1102 * t1053 * t11572;
    let t11580 = t3446 * t10935 * t970;
    let t11582 = t58 * t897;
    (t11568, t11570, t11572, t11574, t11580, t11582)
}
