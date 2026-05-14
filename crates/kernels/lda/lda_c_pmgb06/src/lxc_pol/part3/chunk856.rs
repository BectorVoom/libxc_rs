//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 856/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk856<F: Float>(t5: F, t3559: F, t38: F, t776: F, t247: F, t28: F, t769: F, t8276: F, t3615: F, t63: F, t342: F, t5870: F, t370: F, t8281: F, t2195: F, t642: F, t1: F, t11013: F, t11021: F, t11024: F, t1212: F, t2192: F, t3010: F, t3115: F, t3127: F, t330: F, t332: F, t3537: F, t395: F, t4363: F, t4366: F, t760: F, t8119: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t11225 = 5.84605 * t38 * t776 * t3559;
    let t11227 = t769 * t28 * t247;
    let t11228 = t8276 * t11227;
    let t11229 = 1.9486833333333333 * t11228;
    let t11230 = t63 * t3615;
    let t11231 = t5870 * t342;
    let t11234 = t38 * t370;
    let t11236 = 52.61445 * t11234 * t11231;
    let t11237 = t8281 * t11227;
    let t11259 = 16.0 * t2195 * t642;
    let t11261 = piecewise3(t6, 0.0, -56.0 / 81.0 * t8119 * t760 * t3010 + 16.0 / 9.0 * t3537 * t1 * t11013 + 8.0 / 9.0 * t4363 * t3127 - 4.0 / 3.0 * t1212 * t395 * t332 + 4.0 * t4366 * t11021 - 4.0 / 3.0 * t4366 * t11024 - 2.0 / 9.0 * t2192 * t3115 - 8.0 * t330 * t247 + t11259);
    (t11225, t11229, t11230, t11231, t11236, t11237, t11261)
}
