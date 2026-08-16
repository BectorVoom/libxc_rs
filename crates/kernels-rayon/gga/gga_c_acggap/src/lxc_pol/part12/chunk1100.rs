//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1100/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1100(t1460: f64, t30159: f64, t355: f64, t3706: f64, t7842: f64, t30374: f64, t8606: f64, t1181: f64, t4342: f64, t7351: f64, t7575: f64, t7426: f64, t7569: f64, t8480: f64) -> (f64, f64, f64, f64) {
    let t35585 = t30159 * t7842 * t3706 * t355 * t1460;
    let t35587 = t30374 * t8606;
    let t35591 = t7575 * t1181 * t7351 * t4342;
    let t35594 = t7426 * t8480 * t7569;
    (t35585, t35587, t35591, t35594)
}
