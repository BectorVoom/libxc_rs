//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 877/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk877(t2185: f64, t6422: f64, t2888: f64, t1485: f64, t178: f64, t405: f64, t404: f64, t67: f64, t931: f64, t154: f64, t2226: f64, t385: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6423 = t6422 * t2185;
    let t6424 = t2888 * t6423;
    let t6428 = t178 * t1485 * t405;
    let t6430 = 0.63517063878621832551e-4_f64 * t404 * t6428;
    let t6431 = t67 * t931;
    let t6433 = t154 * t6431 * t2226;
    let t6434 = t385 * t6433;
    (t6423, t6424, t6428, t6430, t6433, t6434)
}
