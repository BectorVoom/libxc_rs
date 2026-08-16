//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1160/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1160(t29470: f64, t7952: f64, t1395: f64, t7275: f64, t573: f64, t7381: f64, t7287: f64, t4261: f64, t7202: f64, t2034: f64, t2043: f64, t28640: f64, t8196: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29471 = t7952 * t29470;
    let t29473 = t1395 * t7275;
    let t29475 = t7381 * t573;
    let t29477 = t1395 * t7287;
    let t29479 = t4261 * t7202;
    let t29480 = t7952 * t29479;
    let t29482 = t2034 * t2043;
    let t29484 = t28640 * t8196;
    (t29471, t29473, t29475, t29477, t29479, t29480, t29482, t29484)
}
