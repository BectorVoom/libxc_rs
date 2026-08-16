//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 419/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk419<F: Float>(t143: F, t1568: F, t164: F, t695: F, t1198: F, t458: F, t479: F, t1124: F, t147: F, t483: F, t485: F, t466: F) -> (F, F, F, F, F, F, F) {
    let t1569 = t143 * t1568;
    let t1573 = F::cast_from(0.06301081444628223_f64) * t695 * t164;
    let t1574 = t1198 * t164;
    let t1577 = F::cast_from(0.06301081444628223_f64) * t458 * t479;
    let t1578 = t1124 * t147;
    let t1581 = F::cast_from(0.006584630109636494_f64) * t1578 * t483 * t485;
    let t1584 = t466 * t479;
    (t1569, t1573, t1574, t1577, t1578, t1581, t1584)
}
