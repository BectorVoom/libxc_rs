//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 788/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk788(t2020: f64, t2260: f64, t599: f64, t8791: f64, t1181: f64, t7413: f64, t1165: f64, t604: f64, t8406: f64, t7346: f64, t8901: f64, t7337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8945 = t2020 * t2260;
    let t8947 = t599 * t8791;
    let t8948 = t1181 * t8947;
    let t8949 = t7413 * t8948;
    let t8952 = t1165 * t604 * t8406;
    let t8953 = t7346 * t8952;
    let t8956 = t1165 * t604 * t8901;
    let t8957 = t7337 * t8956;
    (t8945, t8947, t8948, t8949, t8952, t8953, t8956, t8957)
}
