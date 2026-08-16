//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 573/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk573(t123: f64, t9755: f64, t6118: f64, t1843: f64, t7069: f64, t7064: f64, t1841: f64, t2508: f64, t9719: f64, t9722: f64, t9726: f64, t9731: f64, t9736: f64, t9741: f64, t9745: f64, t9749: f64, t9754: f64) -> (f64, f64, f64) {
    let t9756 = t9755 * t123;
    let t9757 = t9756 * t6118;
    let t9760 = t1843 * t7069;
    let t9762 = 0.64087718584518535698e-3_f64 * t7064 * t9760;
    let t9763 = 0.76905262301422242837e-2_f64 * t2508 * t9719 - 0.53833683610995569986e-1_f64 * t2508 * t9722 - 0.23071578690426672851e-1_f64 * t2508 * t9726 + 0.15381052460284448567e-1_f64 * t2508 * t9731 - 0.46143157380853345701e-1_f64 * t2508 * t9736 + 0.92286314761706691403e-1_f64 * t2508 * t9741 - 0.17090058289204942852e-2_f64 * t1841 * t9745 + 0.85450291446024714263e-3_f64 * t1841 * t9749 + t9754 + 0.25635087433807414279e-2_f64 * t1841 * t9757 + t9762;
    (t9756, t9762, t9763)
}
