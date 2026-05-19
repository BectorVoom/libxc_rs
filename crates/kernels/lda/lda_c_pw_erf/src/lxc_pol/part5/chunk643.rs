//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 643/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk643<F: Float>(t188: F, t473: F, t34: F, t529: F, t2058: F, t331: F, t2055: F, t4659: F, t21: F, t2782: F) -> (F, F, F, F, F, F) {
    let t4956 = t473 * t188;
    let t4957 = t529 * t34;
    let t4998 = F::cast_from(0.017777777777777778_f64) * t331 * t2058;
    let t5000 = F::cast_from(0.002962962962962963_f64) * t331 * t2055;
    let t5017 = F::cast_from(0.015996296296296297_f64) * t4659;
    let t5021 = t21 * t2782;
    (t4956, t4957, t4998, t5000, t5017, t5021)
}
