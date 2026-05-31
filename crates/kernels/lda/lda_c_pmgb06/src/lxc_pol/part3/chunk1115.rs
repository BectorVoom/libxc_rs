//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1115/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1115<F: Float>(t13241: F, t1902: F, t3213: F, t1447: F, t5494: F, t1387: F, t5187: F, t5487: F, t1423: F, t5483: F, t10079: F, t13231: F, t13233: F, t13236: F, t13238: F, t13240: F) -> (F, F, F, F, F, F, F) {
    let t13242 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13241;
    let t13243 = t3213 * t1902;
    let t13244 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t13243;
    let t13245 = t1447 * t5494;
    let t13246 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13245;
    let t13248 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5187 * t1387;
    let t13249 = t1447 * t5487;
    let t13250 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13249;
    let t13251 = t1423 * t5483;
    let t13252 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13251;
    let t13254 = t13231 + t13233 + t13236 + t13238 + t13240 + t13242 - t13244 - t13246 - t13248 - t13250 - t13252 - F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t10079;
    (t13242, t13244, t13246, t13248, t13250, t13252, t13254)
}
