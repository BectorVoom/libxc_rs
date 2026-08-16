//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 565/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk565(t43: f64, t3992: f64, t657: f64, t2618: f64, t2861: f64, t474: f64, t34: f64, t886: f64, t234: f64, t821: f64, t1361: f64, t1364: f64, t39: f64, t47: f64, t818: f64, t824: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t3993 = t3992 * t657;
    let t3994 = 0.10843581300301739842e-1_f64 * t3993;
    let t3995 = 0.21687162600603479684e-1_f64 * t2618;
    let t3996 = t2861 * t474;
    let t3999 = t886 * t34;
    let t4000 = t821 * t234;
    let t4010 = piecewise3(t44, 0.0_f64, -8.0_f64 / 27.0_f64 * t3996 * t818 + 16.0_f64 / 9.0_f64 * t3999 * t4000 + 4.0_f64 / 9.0_f64 * t1361 * t824 + 8.0_f64 / 3.0_f64 * t47 * t821 - 8.0_f64 * t1364 * t39);
    (t3994, t3995, t4000, t4010)
}
