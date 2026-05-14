//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1263/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1263<F: Float>(t10817: F, t11666: F, t11670: F, t14382: F, t14385: F, t14388: F, t14392: F, t14395: F, t14399: F, t14401: F, t14405: F, t14408: F, t18761: F, t18765: F, t18779: F, t18782: F, t18786: F, t18788: F) -> (F,) {
    let t18790 = -0.09579387208203688 * t11666 + 0.017961351015381915 * t18761 - 0.02394846802050922 * t18765 + 0.017961351015381915 * t11670 - 0.003950778065781896 * t14382 - 0.015803112263127583 * t14385 - 0.01185233419734569 * t14388 + 0.026338520438545975 * t14392 + 0.03950778065781896 * t14395 + 0.006935985972286697 * t14399 - 0.051799090195807085 * t14401 - 0.001981710277796199 * t14405 - 0.002972565416694299 * t14408 - t10817 - 0.003950778065781896 * t18779 - 0.003950778065781896 * t18782 - 0.0004954275694490498 * t18786 - 0.06301081444628223 * t18788;
    (t18790,)
}
