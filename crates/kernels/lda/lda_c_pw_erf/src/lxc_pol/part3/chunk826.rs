//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 826/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk826<F: Float>(t39: F, t865: F, t1891: F, t462: F, t169: F, t242: F, t5466: F, t171: F, t4713: F, t2224: F, t632: F, t1143: F, t875: F) -> (F, F, F, F, F, F) {
    let t5745 = t39 * t865;
    let t5750 = F::new(0.2133002709687175) * t462 * t1891;
    let t5760 = t169 * t5466 * t242;
    let t5762 = t171 * t4713;
    let t5768 = F::new(0.06367133154935875) * t169 * t2224 * t632;
    let t5770 = t169 * t875 * t1143;
    (t5745, t5750, t5760, t5762, t5768, t5770)
}
