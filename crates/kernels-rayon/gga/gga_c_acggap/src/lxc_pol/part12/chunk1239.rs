//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1239/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1239(t33743: f64, t33744: f64, t638: f64, t30029: f64, t9171: f64, t33175: f64, t7942: f64, t8406: f64, t2176: f64, t5368: f64, t1620: f64, t8331: f64) -> (f64, f64, f64, f64, f64) {
    let t38343 = 0.10408353825846239354e2_f64 * t33743 * t638 * t33744;
    let t38345 = 0.17347256376410398924e1_f64 * t30029 * t9171;
    let t38348 = 0.17347256376410398924e1_f64 * t7942 * t33175 * t8406;
    let t38361 = t2176 * t5368;
    let t38370 = 0.26341796731742046394e1_f64 * t8331 * t1620;
    (t38343, t38345, t38348, t38361, t38370)
}
