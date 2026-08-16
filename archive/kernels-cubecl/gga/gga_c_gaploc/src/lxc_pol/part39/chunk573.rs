//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 573/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk573<F: Float>(t123: F, t9755: F, t6118: F, t1843: F, t7069: F, t7064: F, t1841: F, t2508: F, t9719: F, t9722: F, t9726: F, t9731: F, t9736: F, t9741: F, t9745: F, t9749: F, t9754: F) -> (F, F, F) {
    let t9756 = t9755 * t123;
    let t9757 = t9756 * t6118;
    let t9760 = t1843 * t7069;
    let t9762 = F::cast_from(0.64087718584518535698e-3_f64) * t7064 * t9760;
    let t9763 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t9719 - F::cast_from(0.53833683610995569986e-1_f64) * t2508 * t9722 - F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t9726 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t9731 - F::cast_from(0.46143157380853345701e-1_f64) * t2508 * t9736 + F::cast_from(0.92286314761706691403e-1_f64) * t2508 * t9741 - F::cast_from(0.17090058289204942852e-2_f64) * t1841 * t9745 + F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t9749 + t9754 + F::cast_from(0.25635087433807414279e-2_f64) * t1841 * t9757 + t9762;
    (t9756, t9762, t9763)
}
