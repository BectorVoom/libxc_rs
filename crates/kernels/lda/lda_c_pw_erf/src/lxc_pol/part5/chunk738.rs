//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 738/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk738<F: Float>(t2748: F, t2752: F, t2755: F, t2759: F, t2761: F, t2944: F, t2950: F, t2989: F, t7330: F, t7332: F, t7333: F, t7336: F, t7350: F, t5695: F, t2325: F, t739: F) -> (F, F, F) {
    let t7351 = t7330 + t7332 + t7333 - t2748 + t2752 - t2755 + t2759 - t2761 - t7336 - t2944 + t2950 - t7350 - t2989;
    let t7353 = 24.0 * t5695;
    let t7354 = t2325 * t739;
    (t7351, t7353, t7354)
}
