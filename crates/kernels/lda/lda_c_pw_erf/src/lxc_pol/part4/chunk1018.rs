//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1018/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1018<F: Float>(t1325: F, t3787: F, t4952: F, t3855: F, t4763: F, t1446: F, t5421: F, t3802: F, t4749: F, t519: F, t5260: F, t13080: F, t1318: F, t4784: F, t1472: F, t5302: F) -> (F, F, F, F, F, F, F) {
    let t13181 = t1325 * t3787 * t4952;
    let t13211 = t4763 * t3855;
    let t13233 = t1446 * t5421;
    let t13238 = t519 * t3802 * t4749;
    let t13241 = t519 * t3802 * t5260;
    let t13244 = t1318 * t13080 * t4784;
    let t13252 = t1472 * t5302;
    (t13181, t13211, t13233, t13238, t13241, t13244, t13252)
}
