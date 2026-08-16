//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2075/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2075(t91137: f64, t26297: f64, t80853: f64, t80855: f64, t26301: f64, t1831: f64, t80866: f64, t131: f64, t6931: f64, t9537: f64, t26322: f64, t236: f64, t26318: f64, t91005: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91138 = 7.0_f64 / 288.0_f64 * t91137;
    let t91140 = t80853 * t80855 * t26297;
    let t91141 = 0.40372756094140390854e-3_f64 * t91140;
    let t91143 = t80853 * t80855 * t26301;
    let t91144 = 0.40372756094140390854e-3_f64 * t91143;
    let t91149 = t80866 * t1831;
    let t91152 = t6931 * t131 * t9537;
    let t91154 = t91152 * t80855 * t26322;
    let t91155 = 0.6728792682356731809e-4_f64 * t91154;
    let t91158 = t91152 * t91005 * t236 * t26318;
    (t91138, t91141, t91144, t91149, t91155, t91158)
}
