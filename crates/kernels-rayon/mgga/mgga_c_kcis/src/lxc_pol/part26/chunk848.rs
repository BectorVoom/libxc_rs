//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 848/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk848(t13577: f64, t5842: f64, t13583: f64, t5836: f64, t5857: f64, t738: f64, t5860: f64, t1441: f64, t1951: f64, t1962: f64, t4016: f64, t1014: f64, t5872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17205 = t13577 * t5842;
    let t17207 = t13583 * t5836;
    let t17237 = t738 * t5857;
    let t17240 = 0.17611111111111111111e-2_f64 * t738 * t5860;
    let t17248 = t1441 * t1951;
    let t17250 = t4016 * t1962;
    let t17259 = t1014 * t5872;
    (t17205, t17207, t17237, t17240, t17248, t17250, t17259)
}
