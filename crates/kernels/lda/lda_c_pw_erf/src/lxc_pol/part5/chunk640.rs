//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 640/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk640<F: Float>(t2344: F, t339: F, t344: F, t6011: F, t87: F, t40: F, t2343: F, t390: F, t3171: F, t3177: F, t3179: F, t3181: F, t3183: F, t3157: F, t3168: F, t3176: F, t5707: F, t5708: F, t5709: F, t5711: F) -> (F, F, F, F, F, F, F) {
    let t6065 = t339 * t2344;
    let t6066 = 4.0 * t6065;
    let t6067 = t344 * t2344;
    let t6068 = 4.0 * t6067;
    let t6069 = t6011 * t87;
    let t6070 = t40 * t6069;
    let t6071 = t2343 * t390;
    let t6072 = t40 * t6071;
    let t6073 = 12.0 * t3171;
    let t6074 = 32.0 * t3177;
    let t6075 = 20.0 * t3179;
    let t6076 = 8.0 * t3181;
    let t6077 = 8.0 * t3183;
    let t6078 = t6066 + t3157 + t5707 - t6068 + t6070 - t5708 - t3168 + t6072 - t5709 + t6073 + t5711 + t3176 + t6074 + t6075 - t6076 - t6077;
    (t6065, t6067, t6069, t6070, t6071, t6072, t6078)
}
