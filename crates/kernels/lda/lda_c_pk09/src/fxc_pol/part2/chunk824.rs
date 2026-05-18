//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 824/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk824<F: Float>(t2258: F, t609: F, t903: F, t3767: F, t3166: F, t623: F, t4008: F, t633: F, t2250: F, t650: F, t1067: F, t2419: F) -> (F, F, F, F, F, F) {
    let t8330 = t903 * t2258 * t609;
    let t8331 = t3767 * t8330;
    let t8334 = t3166 * t2258 * t623;
    let t8338 = t4008 * t2258 * t633;
    let t8342 = t903 * t2250 * t650;
    let t8345 = t2419 * t1067;
    (t8330, t8331, t8334, t8338, t8342, t8345)
}
