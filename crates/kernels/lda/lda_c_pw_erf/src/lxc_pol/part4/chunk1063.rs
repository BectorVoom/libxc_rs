//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1063/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1063<F: Float>(t11387: F, t11389: F, t11391: F, t11401: F, t11403: F, t11405: F, t11463: F, t11465: F, t11397: F, t11399: F, t8373: F, t8382: F, t8386: F, t8389: F, t8393: F, t8397: F, t8400: F, t8405: F, t8414: F, t8417: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15466 = 32.0 * t11387;
    let t15467 = 8.0 * t11389;
    let t15468 = 8.0 * t11391;
    let t15472 = 48.0 * t11401;
    let t15473 = 160.0 * t11403;
    let t15474 = 240.0 * t11405;
    let t15475 = 207.78907079251036 * t11463;
    let t15476 = 4.678578717964164 * t11465;
    let t15477 = -t8373 - t8382 + t8386 - t15466 - t15467 - t15468 - t8389 - t8393 + t8397 - t8400 + 0.6846054806677778 * t8405 - 2.530897186465939 * t11397 + 6.327242966164847 * t11399 - t15472 + t15473 - t15474 + t15475 + t15476 + t8414 + t8417;
    (t15466, t15467, t15468, t15472, t15473, t15474, t15475, t15476, t15477)
}
