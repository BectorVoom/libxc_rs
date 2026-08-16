//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 874/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk874(t1190: f64, t12727: f64, t1159: f64, t3035: f64, t1162: f64, t1165: f64, t3211: f64, t407: f64, t3375: f64, t3445: f64, t3073: f64, t3371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12728 = t12727 * t1190;
    let t12730 = t3035 * t1159;
    let t12731 = t12730 * t1162;
    let t12734 = t12731 * t1165 * t3211 * t407;
    let t12736 = t3375 * t3445;
    let t12738 = t3073 * t3371;
    (t12728, t12730, t12731, t12734, t12736, t12738)
}
