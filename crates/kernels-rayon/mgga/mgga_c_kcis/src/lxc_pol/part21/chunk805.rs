//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 805/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk805(t2822: f64, t2857: f64, t1018: f64, t86: f64, t9526: f64, t1024: f64, t3038: f64, t978: f64, t3368: f64, t2861: f64, t3195: f64, t3230: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9559 = t2822 * t2857;
    let t9562 = t86 * t9526 * t1018;
    let t9563 = t9562 * t1024;
    let t9565 = t3038 * t978;
    let t9568 = t3368 * sigma0;
    let t9572 = t2861 * t3195;
    let t9574 = t2861 * t3230;
    (t9559, t9562, t9563, t9565, t9568, t9572, t9574)
}
