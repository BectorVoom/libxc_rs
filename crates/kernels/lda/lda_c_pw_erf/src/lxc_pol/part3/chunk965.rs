//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 965/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk965<F: Float>(t248: F, t256: F, t4606: F, t5021: F, t3704: F, t665: F, t1124: F, t265: F, t266: F, t3990: F, t640: F, t653: F) -> (F, F, F, F, F) {
    let t11088 = t248 * (-F::new(0.33530864197530863) * t4606 + F::new(1.8360493827160493) * t5021) * t256 / F::new(3.0);
    let t11093 = t665 * t3704;
    let t11097 = F::new(56.0) / F::new(1215.0) * t265 * t266 * t1124;
    let t11098 = t640 * t3990;
    let t11101 = F::new(32.0) / F::new(81.0) * t653 * t3990;
    (t11088, t11093, t11097, t11098, t11101)
}
