//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 503/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk503(t1396: f64, t3797: f64, t1395: f64, t4153: f64, t1392: f64, t540: f64, t1017: f64, t86: f64) -> (f64, f64, f64, f64, f64) {
    let t4154 = t1396 * t3797;
    let t4155 = t1395 * t4154;
    let t4156 = t4153 * t4155;
    let t4158 = t1392 * t540;
    let t4160 = t86 * t1017 * t4158;
    (t4154, t4155, t4156, t4158, t4160)
}
