//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 969/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk969<F: Float>(t242: F, t4120: F, t1159: F, t632: F, t1143: F, t695: F, t9196: F, t4100: F, t1203: F, t2929: F, t466: F, t10953: F, t148: F) -> (F, F, F, F, F, F, F, F) {
    let t11217 = t4120 * t242;
    let t11219 = t1159 * t632;
    let t11222 = F::cast_from(1.0051538464260528_f64) * t695 * t1143;
    let t11223 = t9196 * t242;
    let t11225 = t4100 * t632;
    let t11227 = t1203 * t1143;
    let t11229 = t466 * t2929;
    let t11232 = F::cast_from(0.0837628205355044_f64) * t148 * t10953;
    (t11217, t11219, t11222, t11223, t11225, t11227, t11229, t11232)
}
