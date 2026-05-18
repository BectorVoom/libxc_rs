//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 350/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk350<F: Float>(t1245: F, t940: F, t1243: F, t11: F, t259: F, t43: F) -> (F, F, F, F, F) {
    let t1246 = t1245 * t940;
    let t1247 = t1243 * t1246;
    let t1248 = t11 * t1247;
    let t1250 = t259 * t43;
    let t1251 = F::new(1.0) / t1250;
    (t1246, t1247, t1248, t1250, t1251)
}
