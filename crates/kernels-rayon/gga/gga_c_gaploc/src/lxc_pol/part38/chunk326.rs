//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 326/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk326(t2586: f64, t738: f64, t1841: f64, t1897: f64, t2504: f64, t2508: f64, t2509: f64, t2533: f64, t2538: f64, t2542: f64, t2545: f64, t2550: f64, t2556: f64, t2560: f64, t2565: f64, t2573: f64, t2577: f64, t2583: f64, t270: f64, t650: f64, t681: f64, t938: f64, t949: f64) -> f64 {
    let t2587 = t738 * t2586;
    let t2590 = 0.10254034973522965712e-1_f64 * t650 * t938 + 0.76905262301422242837e-2_f64 * t681 * t938 - 0.76905262301422242837e-2_f64 * t1897 * t2504 + 0.76905262301422242837e-2_f64 * t2508 * t2509 + 0.76905262301422242837e-2_f64 * t270 * t2533 - 0.85450291446024714263e-3_f64 * t1841 * t2538 - 0.23071578690426672851e-1_f64 * t2508 * t2542 - 0.42725145723012357132e-3_f64 * t2545 + 0.32043859292259267849e-3_f64 * t2550 - 0.32043859292259267849e-3_f64 * t2556 + 0.32043859292259267849e-3_f64 * t2560 - 0.32043859292259267849e-3_f64 * t2565 - 0.10254034973522965712e-1_f64 * t650 * t949 - 0.76905262301422242837e-2_f64 * t681 * t949 + 0.76905262301422242837e-2_f64 * t1897 * t2573 + 0.85450291446024714263e-3_f64 * t1841 * t2577 + 0.15381052460284448567e-1_f64 * t2508 * t2583 - 0.76905262301422242837e-2_f64 * t270 * t2587;
    t2590
}
