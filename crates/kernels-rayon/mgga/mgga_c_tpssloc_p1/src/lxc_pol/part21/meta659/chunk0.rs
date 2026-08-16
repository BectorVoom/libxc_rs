//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2460/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2460(t43819: f64, t3311: f64, t409: f64, t3314: f64, t3374: f64, t3399: f64, t440: f64, t3256: f64, t3263: f64, t1094: f64, t11189: f64, t1124: f64, t11349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44053 = 0.31003950617283950618e1_f64 * t43819;
    let t44073 = t3311 * t3311;
    let t44075 = t409 / t44073;
    let t44076 = t3314 * t3314;
    let t44077 = 1.0_f64 / t44076;
    let t44154 = 1.0_f64 / t3399 / t3374;
    let t44155 = t440 * t44154;
    let t44159 = t3256 * t3263;
    let t44162 = t1094 * t11189;
    let t44172 = t1124 * t11349;
    (t44053, t44075, t44077, t44154, t44155, t44159, t44162, t44172)
}
