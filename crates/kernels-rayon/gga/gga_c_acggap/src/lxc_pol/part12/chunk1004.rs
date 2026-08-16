//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1004/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1004(t30090: f64, t8952: f64, t2297: f64, t4210: f64, t13364: f64, t31115: f64, t1: f64, t1170: f64, t2065: f64, t8461: f64, t3196: f64, t1530: f64, t31114: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33916 = t30090 * t8952;
    let t33938 = t2297 * t4210;
    let t33940 = t31115 * t13364 * t33938;
    let t33944 = t1170 * t2065 * t8461 * t1;
    let t33945 = t2297 * t3196;
    let t33947 = t33944 * t13364 * t33945;
    let t33952 = t1530 * t31114;
    (t33916, t33938, t33940, t33944, t33945, t33947, t33952)
}
