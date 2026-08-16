//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1300/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1300(t2512: f64, t9490: f64, t9720: f64, t761: f64, t2517: f64, t718: f64, t2475: f64, t723: f64, t159: f64, t2461: f64, t730: f64, t167: f64, t2478: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9722 = t9720 * t9490 * t2512;
    let t9724 = 0.10389515463408878255e3_f64 * t761 * t9722;
    let t9726 = t718 * t2517;
    let t9729 = 1.0_f64 / t2475 / t723;
    let t9730 = t159 * t9729;
    let t9731 = t2461 * t730;
    let t9733 = 1.0_f64 / t2478 / t167;
    (t9722, t9724, t9726, t9730, t9731, t9733)
}
