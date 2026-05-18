//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 814/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk814<F: Float>(t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t3946: F, t3949: F, t3950: F, t3951: F, t7851: F, t7855: F) -> F {
    let t8202 = F::new(6.0) * t7851 + F::new(6.0) * t7855 - F::new(0.505765839233979) * t3335 - F::new(0.337177226155986) * t3342 + F::new(12.0) * t3384 + F::new(12.0) * t3388 - F::new(12.0) * t3393 + t3946 + t3949 + t3950 - t3951 + F::new(0.505765839233979) * t3317 + F::new(0.505765839233979) * t3319;
    t8202
}
