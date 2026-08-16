//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 246/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk246(t1385: f64, t1386: f64, t1014: f64, t558: f64, t526: f64) -> (f64, f64, f64, f64) {
    let t1387 = t1385 * t1386;
    let t1390 = t1014 * t558;
    let t1391 = 0.16581944444444444444e-2_f64 * t1390;
    let t1392 = 1.0_f64 / t526;
    (t1387, t1390, t1391, t1392)
}
