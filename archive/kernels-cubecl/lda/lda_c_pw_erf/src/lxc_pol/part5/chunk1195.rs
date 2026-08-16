//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1195/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1195<F: Float>(t12975: F, t12999: F, t19256: F, t21675: F, t21676: F, t21677: F, t21678: F, t21680: F, t21681: F, t21683: F, t21685: F, t21687: F, t21692: F) -> F {
    let t21693 = -t12975 - t21675 + t21676 + t21677 - t21678 + t12999 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19256 - t21680 - t21681 - t21683 + t21685 + t21687 + t21692;
    t21693
}
