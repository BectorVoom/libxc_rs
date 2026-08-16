//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1378/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1378(t33801: f64, t33803: f64, t33808: f64, t33810: f64, t33812: f64, t33815: f64, t33818: f64, t33820: f64, t33823: f64, t33825: f64, t33828: f64, t33831: f64) -> (f64, f64) {
    let t36722 = 0.40094868252346065012e-6_f64 * t33801 - 0.21102562238076876322e-7_f64 * t33803 - 0.22098551499687900008e-7_f64 * t33808 - 0.55015711310542948459e-6_f64 * t33810 + 0.40481770833333333336e-4_f64 * t33812 + 0.57920616843011475696e-5_f64 * t33815 - 0.50680539737635041234e-3_f64 * t33818 - 0.34752370105806885418e-3_f64 * t33820 + 0.57920616843011475696e-5_f64 * t33823 - 0.50680539737635041234e-3_f64 * t33825 - 0.34752370105806885418e-3_f64 * t33828;
    let t36723 = 0.69504740211613770836e-3_f64 * t33831;
    (t36722, t36723)
}
