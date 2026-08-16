//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1049/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1049(t3379: f64, t4708: f64, t1165: f64, t3290: f64, t3361: f64, t6138: f64, t1163: f64, t1539: f64, t15560: f64, t3372: f64, t4372: f64, t3431: f64, t4447: f64) -> (f64, f64, f64, f64, f64) {
    let t18305 = t3379 * t4708;
    let t18309 = t3361 * t1165 * t6138 * t3290;
    let t18321 = t1163 * t1165 * t15560 * t1539;
    let t18323 = t3372 * t4372;
    let t18329 = t3431 * t4447;
    (t18305, t18309, t18321, t18323, t18329)
}
