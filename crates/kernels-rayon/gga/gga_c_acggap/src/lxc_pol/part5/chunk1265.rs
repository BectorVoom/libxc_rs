//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1265/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1265(t3431: f64, t5717: f64, t1163: f64, t1165: f64, t4298: f64, t6403: f64, t1181: f64, t22040: f64, t3361: f64, t4643: f64, t5122: f64, t5852: f64) -> (f64, f64, f64, f64) {
    let t23351 = t3431 * t5717;
    let t23355 = t1163 * t1165 * t4298 * t6403;
    let t23359 = t3361 * t1181 * t4643 * t22040;
    let t23363 = t1163 * t1181 * t5852 * t5122;
    (t23351, t23355, t23359, t23363)
}
