//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1023/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1023(t76506: f64, t70506: f64, t70514: f64, t70526: f64, t15669: f64, t2604: f64, t8264: f64, t884: f64, t8946: f64, t70549: f64, t638: f64, t639: f64, t702: f64, t8849: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78546 = 0.23948483403727617128e0_f64 * t76506;
    let t78547 = 0.10248087766267884741e-3_f64 * t70506;
    let t78548 = 0.72042316457491791901e-3_f64 * t70514;
    let t78551 = 0.79828278012425390427e-1_f64 * t70526;
    let t78553 = 0.11974241701863808564e0_f64 * t2604 * t15669;
    let t78556 = 0.11974241701863808564e0_f64 * t884 * t8264 * t8946;
    let t78557 = 0.638468998399467591e-4_f64 * t70549;
    let t78560 = t638 * t639 * t8849 * t702;
    (t78546, t78547, t78548, t78551, t78553, t78556, t78557, t78560)
}
