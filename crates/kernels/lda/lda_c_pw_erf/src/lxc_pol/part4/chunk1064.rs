//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1064/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1064<F: Float>(t8419: F, t479: F, t7032: F, t145: F, t6039: F, t164: F, t7045: F, t11620: F, t11623: F, t9178: F, t9180: F, t9181: F, t9190: F, t9192: F, t9195: F, t9199: F, t9201: F, t9203: F, t9206: F, t9207: F, t9211: F) -> (F, F, F) {
    let t15478 = 2.0 * t8419;
    let t15481 = t7032 * t479;
    let t15483 = t145 * t6039;
    let t15484 = t15483 * t164;
    let t15486 = t7045 * t479;
    let t15496 = t9178 - t9180 - 0.0002373061974330281 * t9181 - 0.02394846802050922 * t9190 - 0.06301081444628223 * t15481 + 0.06301081444628223 * t15484 + 0.06301081444628223 * t15486 - 0.3780648866776934 * t9192 - t9195 + 0.06301081444628223 * t9199 - 0.06301081444628223 * t9201 + 0.1890324433388467 * t9203 + t9206 - 0.031505407223141116 * t9207 + 0.1756220988170676 * t9211 - 0.2520432577851289 * t11620 + 0.06301081444628223 * t11623;
    (t15478, t15483, t15496)
}
