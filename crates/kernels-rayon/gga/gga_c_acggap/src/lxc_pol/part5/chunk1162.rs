//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1162/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1162(t3379: f64, t5608: f64, t1487: f64, t944: f64, t3431: f64, t6271: f64, t1524: f64, t157: f64, t406: f64, t1165: f64, t3194: f64, t4289: f64, t5730: f64) -> (f64, f64, f64, f64, f64) {
    let t20961 = t3379 * t5608;
    let t20963 = t944 * t1487;
    let t20969 = t3431 * t6271;
    let t20972 = t1524 * t406 * t157;
    let t20985 = t3194 * t1165 * t4289 * t5730;
    (t20961, t20963, t20969, t20972, t20985)
}
