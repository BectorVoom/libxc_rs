//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 806/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk806(t2861: f64, t3234: f64, t3318: f64, t1093: f64, t341: f64, t3206: f64, t9429: f64, t2867: f64, t987: f64, t25: f64, t2912: f64, t285: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9576 = t2861 * t3234;
    let t9581 = t2861 * t3318;
    let t9586 = t1093 * t1093;
    let t9587 = 1.0_f64 / t9586;
    let t9588 = t341 * t9587;
    let t9589 = t9588 * sigma0;
    let t9600 = t9429 * t3206;
    let t9608 = t2867 * t987;
    let t9610 = t25 * t2912;
    let t9611 = t285 * t9610;
    (t9576, t9581, t9587, t9588, t9589, t9600, t9608, t9611)
}
