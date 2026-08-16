//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1117/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1117(t2030: f64, t507: f64, t7807: f64, t1488: f64, t2060: f64, t2061: f64, t1165: f64, t20817: f64, t604: f64, t7337: f64, t4975: f64, t7561: f64) -> (f64, f64, f64, f64) {
    let t35856 = t2030 * t507 * t7807;
    let t35860 = t2060 * t1488 * t2061;
    let t35864 = t7337 * t1165 * t604 * t20817;
    let t35866 = t7561 * t4975;
    (t35856, t35860, t35864, t35866)
}
