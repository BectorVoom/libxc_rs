//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1196/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1196<F: Float>(t12113: F, t12114: F, t12115: F, t12116: F, t12117: F, t12119: F, t12121: F, t12123: F, t12125: F, t12129: F, t12131: F, t12135: F, t12138: F, t12142: F, t12145: F, t12149: F, t12153: F, t12159: F, t12164: F, t12168: F, t12170: F, t12174: F, t12179: F) -> (F, F) {
    let t14342 = t12113 - t12114 - t12115 - t12116 - t12117 + t12119 + t12121 + t12123 + t12125 - t12129 + t12131;
    let t14343 = t12135 + t12138 + t12142 + t12145 + t12149 + t12153 + t12159 + t12164 - t12168 - t12170 - t12174 - t12179;
    (t14342, t14343)
}
