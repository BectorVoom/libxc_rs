//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1162/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1162<F: Float>(t2488: F, t3194: F, t493: F, t2493: F, t3177: F, t14316: F, t15257: F, t15258: F, t15259: F, t15260: F, t15261: F, t15263: F, t15268: F, t15270: F, t15273: F, t15278: F, t15280: F, t15282: F) -> (F, F, F) {
    let t15285 = F::new(2.0) / F::new(45.0) * t493 * t3194 * t2488;
    let t15287 = F::new(2.0) / F::new(45.0) * t3177 * t2493;
    let t15288 = -F::new(0.022363485482220676) * t14316 - t15257 + t15258 + t15259 - t15260 - t15261 + t15263 - t15268 + t15270 + t15273 + t15278 - t15280 - t15282 - t15285 - t15287;
    (t15285, t15287, t15288)
}
