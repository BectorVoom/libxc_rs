//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 948/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk948<F: Float>(t12557: F, t1472: F, t5375: F, t1381: F, t1466: F, t5320: F, t571: F, t2153: F, t3742: F, t1318: F, t2151: F, t549: F, t575: F, t825: F, t3794: F, t5394: F) -> (F, F, F, F, F, F, F) {
    let t12558 = 8.0 / 45.0 * t12557;
    let t12560 = 4.0 / 5.0 * t1472 * t5375;
    let t12564 = 4.0 / 5.0 * t571 * t1466 * t5320 * t1381;
    let t12566 = 16.0 / 15.0 * t3742 * t2153;
    let t12570 = 16.0 / 15.0 * t1318 * t2151 * t575 * t549;
    let t12571 = t2151 * t825;
    let t12572 = t571 * t12571;
    let t12573 = 32.0 / 1215.0 * t12572;
    let t12575 = 4.0 / 5.0 * t3794 * t5394;
    (t12558, t12560, t12564, t12566, t12570, t12573, t12575)
}
