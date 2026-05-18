//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1023/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1023<F: Float>(t3279: F, t464: F, t1450: F, t1600: F, t135: F, t1438: F, t144: F, t1461: F, t1489: F, t3247: F, t511: F, t1464: F, t164: F, t170: F) -> (F, F, F, F, F, F) {
    let t10148 = t3279 * t464;
    let t10152 = t1450 * t1600;
    let t10203 = F::new(1.0) / t135 / t1438 * t144;
    let t10216 = t1461 * t1489;
    let t10220 = t3247 * t511;
    let t10230 = F::new(1.0) / t164 / t1464 * t170;
    (t10148, t10152, t10203, t10216, t10220, t10230)
}
