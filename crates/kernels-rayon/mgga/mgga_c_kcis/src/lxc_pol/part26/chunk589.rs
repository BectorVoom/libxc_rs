//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 589/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk589(t1572: f64, t6097: f64, t2079: f64, t4358: f64, t1571: f64, t1347: f64, t1911: f64, t1354: f64, t2084: f64, t1356: f64, t5613: f64, t1919: f64, t3947: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6098 = t6097 * t1572;
    let t6101 = t2079 * t4358;
    let t6102 = t6101 * t1571;
    let t6106 = t1911 * t1347;
    let t6111 = t2084 * t1354;
    let t6114 = t5613 * t1356;
    let t6117 = t1919 * t3947;
    (t6098, t6101, t6102, t6106, t6111, t6114, t6117)
}
