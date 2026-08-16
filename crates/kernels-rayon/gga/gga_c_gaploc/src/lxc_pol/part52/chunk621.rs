//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 621/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk621(t10646: f64, t11578: f64, t11589: f64, t11592: f64, t11596: f64, t11599: f64, t11636: f64, t11672: f64, t11697: f64, t1897: f64, t2508: f64, t270: f64, t3617: f64, t3622: f64, t3627: f64, t3631: f64, t681: f64) -> f64 {
    let t11699 = -0.17090058289204942853e-2_f64 * t10646 - 0.76905262301422242837e-2_f64 * t681 * t3631 - 0.76905262301422242837e-2_f64 * t270 * t11578 - 0.23071578690426672851e-1_f64 * t681 * t3622 + 0.15381052460284448567e-1_f64 * t681 * t3627 + 0.76905262301422242837e-2_f64 * t681 * t3617 + 0.76905262301422242837e-2_f64 * t270 * t11589 - 0.76905262301422242837e-2_f64 * t1897 * t11592 - 0.23071578690426672851e-1_f64 * t2508 * t11596 - 0.53833683610995569986e-1_f64 * t2508 * t11599 + t11636 + t11672 + t11697;
    t11699
}
