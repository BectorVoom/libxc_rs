//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1418/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1418(t11270: f64, t3259: f64, t1094: f64, t11274: f64, t11278: f64, t3262: f64, t3311: f64, t409: f64, t3265: f64, t11277: f64, t11634: f64, t3411: f64) -> (f64, f64, f64, f64, f64) {
    let t43963 = 4.0_f64 * t3259 * t11270;
    let t43964 = t1094 * t11274;
    let t43966 = 0.2069040516770936012e4_f64 * t43964 * t11278;
    let t43969 = t409 / t3311 / t3262;
    let t43970 = t3265 * t3265;
    let t43973 = 0.62071215503128080361e4_f64 * t43969 * t43970 * t11277;
    let t43975 = 0.20779030926817756511e3_f64 * t3411 * t11634;
    (t43963, t43966, t43970, t43973, t43975)
}
