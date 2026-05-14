//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1195/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1195<F: Float>(t10886: F, t118: F, t14527: F, t14529: F, t14533: F, t14535: F, t14539: F, t14541: F, t14543: F, t14545: F, t14547: F, t14549: F, t18059: F, t18062: F, t18064: F, t18066: F, t18069: F, t18071: F, t18076: F) -> (F,) {
    let t18087 = -t10886 - 0.0004954275694490498 * t18059 + 0.06301081444628223 * t18062 + 0.06301081444628223 * t18064 - 0.031505407223141116 * t18066 * t118 - 0.06301081444628223 * t18069 - 0.031505407223141116 * t18071 + 0.1756220988170676 * t14527 + 0.017961351015381915 * t18076 - 0.06301081444628223 * t14529 - 0.06301081444628223 * t14533 - 0.12602162889256446 * t14535 + 0.017961351015381915 * t14539 + 0.1890324433388467 * t14541 - 0.2520432577851289 * t14543 - 0.3780648866776934 * t14545 + 0.06301081444628223 * t14547 + 0.2520432577851289 * t14549;
    (t18087,)
}
