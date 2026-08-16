//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1007/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1007(t35573: f64, t1454: f64, t30148: f64, t30159: f64, t7586: f64, t1460: f64, t355: f64, t3706: f64, t7842: f64, t30374: f64, t8606: f64, t7426: f64, t7569: f64, t8480: f64) -> (f64, f64, f64, f64, f64) {
    let t35574 = 0.31448092289604152068e-2_f64 * t35573;
    let t35580 = t30159 * t7586 * t30148 * t1454;
    let t35581 = 0.12579236915841660827e-2_f64 * t35580;
    let t35585 = t30159 * t7842 * t3706 * t355 * t1460;
    let t35586 = 0.25158473831683321654e-2_f64 * t35585;
    let t35587 = t30374 * t8606;
    let t35594 = t7426 * t8480 * t7569;
    (t35574, t35581, t35586, t35587, t35594)
}
