//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 354/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk354(t2211: f64, t416: f64, t467: f64, t471: f64, t415: f64, t1422: f64, t1423: f64, t2059: f64, t1428: f64, t2083: f64, t457: f64, t1433: f64, t2191: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2212 = t416 * t2211;
    let t2213 = t2212 * t467;
    let t2214 = t2213 * t471;
    let t2215 = t415 * t2214;
    let t2218 = t1422 * t1423 * t2059;
    let t2221 = t1428 * t2083;
    let t2222 = t457 * t2221;
    let t2225 = t1433 * t2191;
    (t2214, t2215, t2218, t2221, t2222, t2225)
}
