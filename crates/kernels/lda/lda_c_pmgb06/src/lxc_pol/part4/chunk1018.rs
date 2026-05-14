//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1018/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1018<F: Float>(t1101: F, t2396: F, t1108: F, t1105: F, t11164: F, t11166: F, t11168: F, t11171: F, t11174: F, t11177: F, t11178: F, t11180: F, t8837: F, t8841: F, t8842: F, t8844: F, t8846: F, t8853: F, t9037: F) -> (F,) {
    let t15045 = t1101 * t2396;
    let t15047 = t1108 * t2396;
    let t15054 = t1105 * t2396;
    let t15056 = -2.3392894490538585 * t11164 - 2050.8037716432814 * t11166 - 69.26343642272586 * t11168 - 1.1696447245269292 * t11171 + 4.0 * t11174 + 2.0 * t11177 - t8837 + 20.0 * t15045 - 32.0 * t15047 + t8841 - 1.1696447245269292 * t11178 - 7.017868347161575 * t11180 + 192.0 * t8842 + 48.0 * t8844 + 96.0 * t8846 + 12.0 * t15054 - t8853 + t9037;
    (t15056,)
}
