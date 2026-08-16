//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 708/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk708(t7952: f64, t7953: f64, t3738: f64, t585: f64, t1468: f64, t1548: f64, t1395: f64, t1552: f64, t7946: f64, t7950: f64) -> (f64, f64, f64, f64, f64) {
    let t7954 = t7952 * t7953;
    let t7956 = t3738 * t585;
    let t7958 = t1468 * t1548;
    let t7960 = t1395 * t1552;
    let t7962 = t7946 / 16.0_f64 - t7950 / 16.0_f64 + t7954 / 24.0_f64 - t7956 / 128.0_f64 + t7958 / 128.0_f64 - t7960 / 96.0_f64;
    (t7954, t7956, t7958, t7960, t7962)
}
