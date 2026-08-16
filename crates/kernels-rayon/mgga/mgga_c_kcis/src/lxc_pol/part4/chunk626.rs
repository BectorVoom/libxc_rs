//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 626/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk626(t3365: f64, t375: f64, t3168: f64, t355: f64, t381: f64, t389: f64, t143: f64, t3038: f64, t245: f64, t365: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3366 = t375 * t3365;
    let t3368 = t3168 * t355;
    let t3369 = t3368 * t381;
    let t3370 = t3369 * t389;
    let t3372 = t3038 * t143;
    let t3381 = t365 * t245;
    (t3366, t3368, t3369, t3370, t3372, t3381)
}
