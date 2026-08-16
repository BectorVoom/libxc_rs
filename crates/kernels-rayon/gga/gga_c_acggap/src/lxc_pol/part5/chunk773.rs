//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 773/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk773(t1323: f64, t4593: f64, t1327: f64, t1314: f64, t1318: f64, t1814: f64, t435: f64) -> (f64, f64, f64, f64, f64) {
    let t5910 = t4593 * t1323;
    let t5913 = t4593 * t1327;
    let t5916 = t4593 * t1314;
    let t5919 = t4593 * t1318;
    let t5922 = t435 * t1814;
    (t5910, t5913, t5916, t5919, t5922)
}
