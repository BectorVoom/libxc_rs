//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 583/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk583(t2701: f64, t646: f64, t918: f64, t3343: f64, t1026: f64, t933: f64, t937: f64, t1081: f64, t954: f64, t3314: f64, t3316: f64, t3318: f64, t3323: f64, t3331: f64, t3334: f64, t3338: f64, t3341: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3345 = t646 * t918 * t2701;
    let t3346 = t3343 * t3345;
    let t3348 = t933 * t1026;
    let t3349 = t3348 * t937;
    let t3351 = t1081 * t954;
    let t3353 = 0.50602213541666666669e-5_f64 * t3314 - 0.25301106770833333334e-4_f64 * t3316 - 0.3243554543208642639e-2_f64 * t3318 - 0.25013570439533790734e-8_f64 * t3323 - 0.12309827972211511188e-7_f64 * t3331 + 0.10567613244746075633e-6_f64 * t3334 + 0.86880925264517213544e-4_f64 * t3338 + 0.86880925264517213544e-4_f64 * t3341 - 0.12872857093359300474e-5_f64 * t3346 + 0.11594181388521408695e-4_f64 * t3349 - 0.2318836277704281739e-4_f64 * t3351;
    (t3345, t3346, t3348, t3349, t3351, t3353)
}
