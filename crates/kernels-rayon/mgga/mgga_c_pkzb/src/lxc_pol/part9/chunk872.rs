//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 872/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk872(t6366: f64, t6368: f64, t2382: f64, t2434: f64, t2381: f64, t1478: f64, t154: f64, t386: f64, t385: f64, t465: f64, t931: f64, t179: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6369 = t6366 * t6368;
    let t6372 = t2434 * t2382;
    let t6373 = t2381 * t6372;
    let t6377 = t154 * t1478 * t386;
    let t6379 = 5.0_f64 / 1296.0_f64 * t385 * t6377;
    let t6380 = t465 * t931;
    let t6382 = t179 * t6380 * t824;
    (t6369, t6372, t6373, t6377, t6379, t6380, t6382)
}
