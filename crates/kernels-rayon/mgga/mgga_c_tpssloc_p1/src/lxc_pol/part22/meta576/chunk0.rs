//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2085/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2085(t43819: f64, t3330: f64, t3355: f64, t427: f64, t43776: f64, t1174: f64, t1186: f64, t2402: f64, t457: f64, t625: f64, t221: f64, t456: f64, t461: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44320 = 0.17757530864197530864e0_f64 * t43819;
    let t44348 = 0.18467901234567901234e0_f64 * t43819;
    let t44361 = t427 / t3355 / t3330;
    let t44466 = 220.0_f64 / 81.0_f64 * t43776;
    let t44478 = t1174 * t2402 * t1186;
    let t44483 = t625 * t457;
    let t44487 = 0.82304526748971193413e-3_f64 * t456 * t221 * t44483 * t461;
    (t44320, t44348, t44361, t44466, t44478, t44483, t44487)
}
