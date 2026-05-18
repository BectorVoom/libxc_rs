//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 573/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk573<F: Float>(t161: F, t2983: F, t340: F, t838: F, t168: F, t609: F, t623: F, t121: F, t3141: F, t633: F, t707: F, t2972: F, t3194: F) -> (F, F, F, F, F, F) {
    let t4028 = t161 * t2983;
    let t4030 = t838 * t340;
    let t4031 = t168 * t609;
    let t4032 = t4031 * t623;
    let t4033 = t121 * t4032;
    let t4034 = t4030 * t4033;
    let t4037 = t838 * t3141;
    let t4038 = t4031 * t633;
    let t4039 = t707 * t4038;
    let t4040 = t4037 * t4039;
    let t4042 = t2972 * t3194;
    (t4028, t4030, t4034, t4037, t4040, t4042)
}
