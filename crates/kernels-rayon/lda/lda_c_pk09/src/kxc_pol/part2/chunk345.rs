//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 345/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk345(t1672: f64, t525: f64, t1222: f64, t515: f64, t476: f64, t508: f64, t520: f64, t10: f64, t1240: f64, t437: f64, t440: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1674 = 6.211752672544321_f64 * t525 * t1672;
    let t1675 = t1222 * t515;
    let t1677 = 0.013716887843283197_f64 * t476 * t1675;
    let t1679 = 1.6457779058161184_f64 * t508 * t1672;
    let t1680 = t520 * t520;
    let t1681 = 1.0_f64 / t1680;
    let t1683 = t1240 * t437 * t10;
    let t1684 = t1683 * t440;
    (t1674, t1675, t1677, t1679, t1680, t1681, t1683, t1684)
}
