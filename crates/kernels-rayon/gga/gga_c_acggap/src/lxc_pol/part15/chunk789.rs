//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 789/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk789(t360: f64, t525: f64, t1181: f64, t604: f64, t2068: f64, t1165: f64, t7351: f64, t8906: f64, t8402: f64, t1967: f64, t2310: f64, t2290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8960 = t525 * t360;
    let t8962 = t1181 * t604 * t8960;
    let t8963 = t2068 * t8962;
    let t8966 = t1165 * t7351 * t8906;
    let t8967 = t2068 * t8966;
    let t8970 = t1165 * t604 * t8402;
    let t8971 = t2068 * t8970;
    let t8973 = t1967 * t2310;
    let t8975 = t1967 * t2290;
    (t8960, t8962, t8963, t8966, t8967, t8970, t8971, t8973, t8975)
}
