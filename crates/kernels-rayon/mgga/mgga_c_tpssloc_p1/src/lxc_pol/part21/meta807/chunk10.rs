//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2820/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2820(t4303: f64, t16625: f64, t193: f64, t202: f64, t2522: f64, t2553: f64, t2752: f64, t4314: f64, t5527: f64, t59029: f64, t59031: f64, t59033: f64, t59034: f64, t59035: f64, t59038: f64, t59040: f64, t59043: f64, t59046: f64, t59049: f64, t9470: f64) -> f64 {
    let t59609 = t4303 * t4303;
    let t59614 = -2.0_f64 * t193 * t202 * t2752 * t59609 - 3.0_f64 * t16625 * t2522 * t2553 - 6.0_f64 * t4314 * t5527 * t9470 - t59029 + t59031 + t59033 + t59034 + t59035 + t59038 + t59040 + t59043 - t59046 - t59049;
    t59614
}
