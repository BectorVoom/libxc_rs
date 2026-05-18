//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 236/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk236<F: Float>(t286: F, t684: F, t159: F, t285: F, t465: F, t147: F, t477: F, t281: F, t462: F) -> (F, F, F, F, F) {
    let t686 = F::new(0.019957056683757683) * t684 * t286;
    let t688 = t465 * t159 * t285;
    let t692 = t147 * t477 * t285;
    let t694 = F::new(0.01197423401025461) * t281 * t692;
    let t695 = t462 * t147;
    (t686, t688, t692, t694, t695)
}
