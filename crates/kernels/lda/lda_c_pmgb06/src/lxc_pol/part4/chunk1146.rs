//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1146/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1146<F: Float>(t103: F, t13399: F, t13407: F, t14162: F, t14170: F, t14181: F, t14183: F, t14185: F, t14187: F, t14189: F, t16359: F, t3358: F, t9530: F, t9532: F, t9552: F, t161: F, t166: F, t17047: F, t17077: F, t17097: F, t17121: F, t17159: F, t17195: F, t17229: F, t518: F) -> (F,) {
    let t17245 = 0.035555555555555556 * t103 * t3358 * t16359 + 1.135737037037037 * t13399 + 0.07464938271604939 * t13407 + 0.2725925925925926 * t14162 + 0.03950617283950617 * t14170 + 0.05925925925925926 * t14181 - 0.009876543209876543 * t14183 - 0.017777777777777778 * t14185 + 0.07111111111111111 * t14187 + 0.002962962962962963 * t14189 - 0.015996296296296297 * t9530 - 0.010664197530864198 * t9532 + 0.07464938271604939 * t9552;
    let t17252 = t161 * t166 * t518 * (t17047 + t17077 + t17097 + t17121 + t17159 + t17195 + t17229 + t17245) / 30.0;
    (t17252,)
}
