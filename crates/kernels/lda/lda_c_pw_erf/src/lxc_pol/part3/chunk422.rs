//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 422/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk422<F: Float>(t1098: F, t1138: F, t1597: F, t163: F, t169: F, t234: F, t717: F, t299: F, t616: F, t230: F, t598: F, t226: F, t610: F) -> (F, F, F, F, F) {
    let t1599 = F::new(0.0004954275694490498) * t1098 * t1138 * t1597;
    let t1603 = F::new(0.02394846802050922) * t169 * t717 * t234 * t163;
    let t1606 = t169 * t299 * t616 * t163;
    let t1608 = t598 * t230;
    let t1611 = F::new(8.0) / F::new(3.0) * t226 * t610;
    (t1599, t1603, t1606, t1608, t1611)
}
