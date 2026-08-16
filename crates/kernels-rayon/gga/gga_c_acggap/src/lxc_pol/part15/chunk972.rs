//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 972/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk972(t30343: f64, t30347: f64, t30811: f64, t4273: f64, t129: f64, t507: f64, t7585: f64, t7587: f64, t30546: f64, t8477: f64, t1967: f64, t8561: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34338 = 0.10718504529517434243e-2_f64 * t30343;
    let t34339 = 0.42874018118069736972e-3_f64 * t30347;
    let t34340 = t30811 * t4273;
    let t34345 = t129 * t507;
    let t34347 = t7585 * t34345 * t7587;
    let t34349 = t30546 * t8477;
    let t34351 = t1967 * t8561;
    (t34338, t34339, t34340, t34345, t34347, t34349, t34351)
}
