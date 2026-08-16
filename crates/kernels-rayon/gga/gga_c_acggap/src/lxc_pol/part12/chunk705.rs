//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 705/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk705(t1084: f64, t1181: f64, t7351: f64, t7564: f64, t1111: f64, t604: f64, t7426: f64, t2070: f64, t7433: f64, t2450: f64, t7336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7566 = t1181 * t7351 * t1084;
    let t7567 = t7564 * t7566;
    let t7569 = t604 * t1111;
    let t7570 = t1181 * t7569;
    let t7571 = t7426 * t7570;
    let t7572 = 0.42874018118069736972e-3_f64 * t7571;
    let t7573 = t7433 * t2070;
    let t7574 = 0.12862205435420921092e-2_f64 * t7573;
    let t7575 = t2450 * t7336;
    (t7566, t7567, t7569, t7570, t7571, t7572, t7573, t7574, t7575)
}
