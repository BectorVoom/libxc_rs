//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1030/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1030(t1008: f64, t4849: f64, t4853: f64, t4878: f64, t1181: f64, t3290: f64, t3391: f64, t6337: f64, t3409: f64, t4406: f64, t1165: f64, t12991: f64, t3355: f64, t540: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17619 = t1008 * t4849;
    let t17621 = t1008 * t4853;
    let t17623 = t1008 * t4878;
    let t17627 = t3391 * t1181 * t6337 * t3290;
    let t17631 = t3409 * t4406;
    let t17635 = t12991 * t1165 * t540 * t3355;
    (t17619, t17621, t17623, t17627, t17631, t17635)
}
