//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1022/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1022(t1220: f64, t2349: f64, t154: f64, t2347: f64, t3026: f64, t385: f64, t7945: f64, t907: f64, t1167: f64, t6446: f64, t2344: f64, t2387: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8325 = t1220 * t2349 / 54.0_f64;
    let t8329 = t154 * t2347 * t3026;
    let t8331 = t385 * t8329 / 144.0_f64;
    let t8333 = t154 * t907 * t7945;
    let t8339 = t154 * t6446 * t1167;
    let t8340 = t385 * t8339;
    let t8342 = t1220 * t2344;
    let t8344 = t1167 * t2387;
    (t8325, t8331, t8333, t8340, t8342, t8344)
}
