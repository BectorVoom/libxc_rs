//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1013/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1013(t14050: f64, t4921: f64, t3382: f64, t4695: f64, t1165: f64, t3361: f64, t3759: f64, t4267: f64, t1170: f64, t15392: f64) -> (f64, f64, f64, f64) {
    let t17118 = t14050 * t4921;
    let t17120 = t3382 * t4695;
    let t17128 = t3361 * t1165 * t4267 * t3759;
    let t17139 = t1170 * t15392;
    (t17118, t17120, t17128, t17139)
}
