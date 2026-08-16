//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 615/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk615(t3401: f64, t50: f64, t581: f64, t3396: f64, t1034: f64) -> (f64, f64, f64, f64, f64) {
    let t3402 = t50 * t3401;
    let t3403 = t581 * t3402;
    let t3406 = t50 * t3396;
    let t3407 = t581 * t3406;
    let t3410 = t1034 * t1034;
    (t3402, t3403, t3406, t3407, t3410)
}
