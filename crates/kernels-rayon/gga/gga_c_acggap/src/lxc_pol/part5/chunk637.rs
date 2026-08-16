//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 637/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk637(t1629: f64, t4199: f64, t945: f64, t1651: f64, t930: f64, t322: f64, t407: f64) -> (f64, f64, f64, f64) {
    let t4200 = t1629 * t4199;
    let t4203 = t1629 * t945;
    let t4206 = t1651 * t930;
    let t4210 = t407 * t322;
    (t4200, t4203, t4206, t4210)
}
