//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 852/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk852<F: Float>(t4574: F, t811: F, t5165: F, t4722: F, t784: F, t5146: F, t549: F, t820: F, t184: F, t4387: F, t4389: F, t4391: F) -> (F, F, F, F, F, F, F, F) {
    let t6748 = t4574 * t811;
    let t6752 = t5165 * t811;
    let t6762 = t4722 * t784;
    let t6766 = t5146 * t784;
    let t6850 = t549 * t820;
    let t6851 = t6850 * t184;
    let t7324 = F::cast_from(0.0007324622014701264_f64) * t4387;
    let t7325 = F::cast_from(1.7544670192365612_f64) * t4389;
    let t7326 = F::cast_from(51.94726769812759_f64) * t4391;
    (t6748, t6752, t6762, t6766, t6851, t7324, t7325, t7326)
}
