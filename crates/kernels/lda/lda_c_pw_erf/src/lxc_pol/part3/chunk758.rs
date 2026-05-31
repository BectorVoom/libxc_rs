//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 758/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk758<F: Float>(t2168: F, t3794: F, t1472: F, t2140: F, t1446: F, t2188: F, t4804: F, t1443: F, t4738: F, t2183: F, t2193: F, t4753: F) -> (F, F, F, F, F, F, F) {
    let t4915 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3794 * t2168;
    let t4917 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t1472 * t2140;
    let t4919 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1446 * t2188;
    let t4921 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4804 * t2168;
    let t4923 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4738 * t1443;
    let t4925 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3794 * t2183;
    let t4927 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4753 * t2193;
    (t4915, t4917, t4919, t4921, t4923, t4925, t4927)
}
