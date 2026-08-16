//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1262/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1262(t16090: f64, t469: f64, t45: f64, t5586: f64, t4370: f64, t5595: f64, t1893: f64, t3860: f64, t3863: f64, t11536: f64, t1919: f64, t11539: f64, t3919: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16092 = 0.62182e-1_f64 * t16090 * t469;
    let t16093 = t45 * t5586;
    let t16100 = t5595 * t4370;
    let t16103 = t1893 * t3860;
    let t16105 = 2.0_f64 * t16103 * t3863;
    let t16106 = t11536 * t1919;
    let t16107 = t11539 * t3919;
    (t16092, t16093, t16100, t16105, t16106, t16107)
}
