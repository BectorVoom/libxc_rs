//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1069/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1069(t1064: f64, t13495: f64, t10114: f64, t167: f64, t1071: f64, t2622: f64, t1056: f64, t1079: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13496 = t1064 * t13495;
    let t13499 = t10114 * t167;
    let t13501 = t2622 * t1071;
    let t13502 = t13501 * t167;
    let t13504 = t1056 * t13495;
    let t13507 = t1079 * t13495;
    let t13510 = t1071 * t167;
    let t13511 = t13510 * t829;
    (t13496, t13499, t13502, t13504, t13507, t13511)
}
