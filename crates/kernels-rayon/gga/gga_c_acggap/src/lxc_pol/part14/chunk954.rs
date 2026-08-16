//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 954/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk954(t3201: f64, t8489: f64, t1980: f64, t7458: f64, t30090: f64, t8952: f64, t2297: f64, t4210: f64, t13364: f64, t31115: f64, t1: f64, t1170: f64, t2065: f64, t8461: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33901 = t3201 * t8489;
    let t33903 = t1980 * t7458 * t33901;
    let t33904 = 0.28582678745379824648e-3_f64 * t33903;
    let t33916 = t30090 * t8952;
    let t33938 = t2297 * t4210;
    let t33940 = t31115 * t13364 * t33938;
    let t33941 = 0.10718504529517434243e-2_f64 * t33940;
    let t33944 = t1170 * t2065 * t8461 * t1;
    (t33901, t33904, t33916, t33938, t33941, t33944)
}
