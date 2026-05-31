//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 938/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk938<F: Float>(t13243: F, t1555: F, t1848: F, t3155: F, t831: F, t177: F, t2911: F, t2918: F, t1531: F, t1593: F, t1827: F, t947: F) -> (F, F, F, F, F, F, F) {
    let t13244 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t13243;
    let t13291 = t1848 * t1555;
    let t13292 = t13291 / F::cast_from(45.0_f64);
    let t13294 = t831 * t3155;
    let t13295 = t13294 / F::cast_from(45.0_f64);
    let t13300 = t177 * t2911;
    let t13304 = t177 * t2918;
    let t13308 = t1593 * t1531;
    let t13370 = t947 * t1827;
    (t13244, t13292, t13295, t13300, t13304, t13308, t13370)
}
