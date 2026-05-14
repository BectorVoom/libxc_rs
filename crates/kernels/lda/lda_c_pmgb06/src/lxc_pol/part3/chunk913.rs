//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 913/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk913<F: Float>(t432: F, t5115: F, t517: F, t5415: F, t161: F, t166: F, t529: F, t2887: F, t831: F, t531: F, t5432: F, t1641: F, t1848: F, t4803: F, t486: F, t490: F) -> (F, F, F, F, F, F, F) {
    let t12259 = t432 * t5115;
    let t12260 = 2.0 / 15.0 * t12259;
    let t12261 = t5415 * t517;
    let t12265 = t161 * t166 * t12261 * t529 / 10.0;
    let t12267 = t831 * t2887 / 10.0;
    let t12269 = t5432 * t531 / 10.0;
    let t12271 = t1848 * t1641 / 5.0;
    let t12273 = t486 * t4803 / 5.0;
    let t12274 = t5432 * t490;
    (t12260, t12265, t12267, t12269, t12271, t12273, t12274)
}
