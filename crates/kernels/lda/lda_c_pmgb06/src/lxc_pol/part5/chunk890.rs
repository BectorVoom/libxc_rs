//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 890/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk890<F: Float>(t3248: F, t517: F, t122: F, t227: F, t8088: F, t199: F, t2778: F, t4182: F, t610: F, t3993: F, t1135: F, t566: F) -> (F, F, F, F, F, F) {
    let t10445 = t3248 * t517;
    let t10472 = F::cast_from(0.9079060239445599_f64) * t122 * t8088 * t227;
    let t10479 = F::cast_from(2.0103076928521055_f64) * t2778 * t199;
    let t10487 = t122 * t4182 * t610;
    let t10492 = t3993 * t199;
    let t10494 = t1135 * t566;
    (t10445, t10472, t10479, t10487, t10492, t10494)
}
