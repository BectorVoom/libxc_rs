//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 683/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk683<F: Float>(t3183: F, t3157: F, t3168: F, t3176: F, t5707: F, t5708: F, t5709: F, t5711: F, t6066: F, t6068: F, t6070: F, t6072: F, t6073: F, t6074: F, t6075: F, t6076: F) -> F {
    let t6077 = F::new(8.0) * t3183;
    let t6078 = t6066 + t3157 + t5707 - t6068 + t6070 - t5708 - t3168 + t6072 - t5709 + t6073 + t5711 + t3176 + t6074 + t6075 - t6076 - t6077;
    t6078
}
