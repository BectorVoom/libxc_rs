//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 529/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk529<F: Float>(t3428: F, t3444: F, t3453: F, t3290: F, t719: F, t192: F, t3230: F, t3233: F, t159: F, t733: F, t142: F, t3163: F, t35: F, t68: F, t889: F, t3161: F, t62: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3963 = 16.0 * t3428;
    let t3967 = 6.0 * t3444;
    let t3969 = 16.0 * t3453;
    let t3983 = 2.2140749178833072 * t719 * t3290;
    let t3984 = t192 * t3230;
    let t3986 = t192 * t3233;
    let t3990 = t159 * t733;
    let t3991 = t3990 * t142;
    let t3993 = 9.87466743489671 * t3991 * t3163;
    let t3994 = t35 * t35;
    let t3995 = 1.0 / t3994;
    let t3996 = t3995 * t68;
    let t3997 = t3996 * t889;
    let t3998 = t62 * t3161;
    (t3963, t3967, t3969, t3983, t3984, t3986, t3993, t3995, t3997, t3998)
}
