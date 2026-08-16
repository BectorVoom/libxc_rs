//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 962/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk962(t1024: f64, t9562: f64, t3038: f64, t978: f64, t3368: f64, t2861: f64, t3195: f64, t3230: f64, t3234: f64, t3318: f64, t1093: f64, t341: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9563 = t9562 * t1024;
    let t9565 = t3038 * t978;
    let t9568 = t3368 * sigma0;
    let t9572 = t2861 * t3195;
    let t9574 = t2861 * t3230;
    let t9576 = t2861 * t3234;
    let t9581 = t2861 * t3318;
    let t9586 = t1093 * t1093;
    let t9587 = 1.0_f64 / t9586;
    let t9588 = t341 * t9587;
    (t9563, t9565, t9568, t9572, t9574, t9576, t9581, t9587, t9588)
}
