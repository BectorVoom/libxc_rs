//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1072/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1072<F: Float>(t10162: F, t2187: F, t519: F, t1472: F, t5375: F, t1381: F, t1466: F, t5320: F, t571: F, t2153: F, t3742: F, t1318: F, t2151: F, t549: F, t575: F) -> (F, F, F, F, F) {
    let t12557 = t519 * t10162 * t2187;
    let t12558 = F::new(8.0) / F::new(45.0) * t12557;
    let t12560 = F::new(4.0) / F::new(5.0) * t1472 * t5375;
    let t12564 = F::new(4.0) / F::new(5.0) * t571 * t1466 * t5320 * t1381;
    let t12566 = F::new(16.0) / F::new(15.0) * t3742 * t2153;
    let t12570 = F::new(16.0) / F::new(15.0) * t1318 * t2151 * t575 * t549;
    (t12558, t12560, t12564, t12566, t12570)
}
