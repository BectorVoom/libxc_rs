//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 556/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk556(t3540: f64, t485: f64, t221: f64, t3426: f64, t456: f64, t1176: f64, t3247: f64, t3242: f64, t3439: f64, t121: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3542 = t485 * t3540 / 13824.0_f64;
    let t3545 = t221 * t3426;
    let t3547 = t456 * t3545 / 432.0_f64;
    let t3555 = t1176 * t3247;
    let t3560 = t3439 * t3242;
    let t3570 = t121 * t486;
    (t3542, t3545, t3547, t3555, t3560, t3570)
}
