//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1136/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1136(t7426: f64, t7569: f64, t8480: f64, t7433: f64, t8481: f64, t4680: f64, t8463: f64, t8652: f64, t34161: f64, t8465: f64, t31421: f64, t1992: f64, t7585: f64, t7842: f64, t8402: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35594 = t7426 * t8480 * t7569;
    let t35595 = 0.42874018118069736972e-3_f64 * t35594;
    let t35596 = t7433 * t8481;
    let t35597 = 0.12862205435420921092e-2_f64 * t35596;
    let t35599 = t8463 * t4680 * t8652;
    let t35601 = t34161 * t8465;
    let t35602 = 0.56606566121287473722e-1_f64 * t35601;
    let t35603 = 0.22921875e-1_f64 * t31421;
    let t35608 = t7585 * t7842 * t1992 * t8402;
    (t35595, t35597, t35599, t35602, t35603, t35608)
}
