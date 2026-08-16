//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 795/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk795<F: Float>(t5334: F, t577: F, t3745: F, t799: F, t3783: F, t798: F, t519: F, t3762: F, t825: F, t571: F, t3742: F, t826: F) -> (F, F, F, F, F, F, F) {
    let t5336 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5334 * t577;
    let t5338 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t3745 * t799;
    let t5339 = t3783 * t798;
    let t5340 = t519 * t5339;
    let t5341 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t5340;
    let t5342 = t3762 * t825;
    let t5343 = t571 * t5342;
    let t5344 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t5343;
    let t5346 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t3742 * t826;
    (t5336, t5338, t5339, t5341, t5342, t5344, t5346)
}
