//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 344/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk344<F: Float>(t1333: F, t219: F, t1251: F, t197: F, t1237: F, t174: F, t205: F, t325: F, t560: F, t56: F, t573: F) -> (F, F, F, F, F, F) {
    let t1334 = t219 * t1333;
    let t1339 = t197 * t1251;
    let t1345 = t174 * t1237 * t205;
    let t1346 = F::cast_from(0.047988888888888886_f64) * t1345;
    let t1347 = t325 * t560;
    let t1349 = t56 * t573;
    (t1334, t1339, t1345, t1346, t1347, t1349)
}
