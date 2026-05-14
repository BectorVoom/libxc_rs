//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1135/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1135<F: Float>(t10172: F, t12975: F, t21614: F, t21617: F, t21622: F, t21624: F, t21664: F, t21665: F, t21668: F, t21675: F, t21676: F, t21677: F, t21678: F, t12999: F, t13049: F, t13052: F, t13359: F, t21680: F, t21681: F, t21683: F, t21685: F, t21687: F, t21692: F, t21694: F, t21695: F, t21696: F) -> (F, F) {
    let t23257 = -t21614 + t21617 - t21622 + t21624 - t10172 + t21664 - t21665 - t21668 - t12975 - t21675 + t21676 + t21677 - t21678;
    let t23258 = t12999 - t21680 - t21681 - t21683 + t21685 + t21687 + t21692 + t13049 + t13052 + t21694 + t21695 - t21696 - t13359;
    (t23257, t23258)
}
