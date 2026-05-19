//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 666/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk666<F: Float>(t1891: F, t462: F, t169: F, t242: F, t5466: F, t2224: F, t632: F, t1143: F, t875: F, t1904: F, t299: F, t2220: F) -> (F, F, F, F, F, F, F) {
    let t5750 = F::cast_from(0.2133002709687175_f64) * t462 * t1891;
    let t5760 = t169 * t5466 * t242;
    let t5768 = F::cast_from(0.06367133154935875_f64) * t169 * t2224 * t632;
    let t5770 = t169 * t875 * t1143;
    let t5772 = t299 * t1904;
    let t5775 = F::cast_from(0.10611888591559791_f64) * t169 * t5772 * t242;
    let t5777 = t169 * t2220 * t632;
    (t5750, t5760, t5768, t5770, t5772, t5775, t5777)
}
