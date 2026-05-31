//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1073/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1073<F: Float>(t2151: F, t825: F, t571: F, t3794: F, t5394: F, t12543: F, t12545: F, t12547: F, t12549: F, t12551: F, t12555: F, t12558: F, t12560: F, t12564: F, t12566: F, t12570: F) -> (F, F, F) {
    let t12571 = t2151 * t825;
    let t12572 = t571 * t12571;
    let t12573 = F::cast_from(32.0_f64) / F::cast_from(1215.0_f64) * t12572;
    let t12575 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t3794 * t5394;
    let t12576 = t12543 - t12545 + t12547 + t12549 + t12551 - t12555 - t12558 + t12560 + t12564 - t12566 - t12570 + t12573 - t12575;
    (t12573, t12575, t12576)
}
