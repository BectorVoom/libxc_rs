//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 813/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk813(t3408: f64, t72: f64, t11280: f64, t1526: f64, t1527: f64, t15567: f64, t16631: f64, t16634: f64, t16641: f64, t16644: f64, t16649: f64, t3313: f64, t3323: f64, t3338: f64, t3414: f64, t342: f64, t343: f64, t8759: f64, t8761: f64, t8764: f64) -> f64 {
    let t16654 = t72 * t3408;
    let t16658 = t3313 + t3414 + t8759 - t8761 / 36.0_f64 - t8764 / 12.0_f64 - t16631 / 36.0_f64 - t15567 * t16634 / 9.0_f64 - t1526 * t1527 * t3323 / 12.0_f64 + t15567 * t16641 / 6.0_f64 - t1526 * t11280 * t16644 / 6.0_f64 - t16649 / 12.0_f64 - t1526 * t1527 * t3338 / 12.0_f64 - t342 * t343 * t16654 / 4.0_f64;
    t16658
}
