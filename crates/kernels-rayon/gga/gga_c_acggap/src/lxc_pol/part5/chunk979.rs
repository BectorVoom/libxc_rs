//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 979/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk979(t1163: f64, t1165: f64, t16020: f64, t540: f64, t3382: f64, t4991: f64, t4983: f64, t14047: f64, t4277: f64, t1111: f64, t15947: f64, t3361: f64) -> (f64, f64, f64, f64, f64) {
    let t16023 = t1163 * t1165 * t540 * t16020;
    let t16025 = t3382 * t4991;
    let t16044 = t3382 * t4983;
    let t16051 = t14047 * t4277;
    let t16055 = t3361 * t1165 * t15947 * t1111;
    (t16023, t16025, t16044, t16051, t16055)
}
