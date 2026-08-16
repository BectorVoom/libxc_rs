//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2229/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2229(t26257: f64, t3872: f64, t1831: f64, t80869: f64, t22783: f64, t5314: f64, t26297: f64, t80853: f64, t80855: f64, t26301: f64, t22788: f64, t16333: f64, t6952: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91133 = t26257 * t3872;
    let t91135 = t80869 * t1831;
    let t91136 = 7.0_f64 / 288.0_f64 * t91135;
    let t91137 = t22783 * t5314;
    let t91138 = 7.0_f64 / 288.0_f64 * t91137;
    let t91140 = t80853 * t80855 * t26297;
    let t91141 = 0.40372756094140390854e-3_f64 * t91140;
    let t91143 = t80853 * t80855 * t26301;
    let t91144 = 0.40372756094140390854e-3_f64 * t91143;
    let t91145 = t22788 * t5314;
    let t91147 = t6952 * t16333;
    (t91133, t91136, t91138, t91141, t91144, t91145, t91147)
}
