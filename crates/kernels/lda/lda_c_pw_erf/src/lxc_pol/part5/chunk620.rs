//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 620/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk620<F: Float>(t1904: F, t473: F, t483: F, t485: F, t1131: F, t1910: F, t142: F, t1832: F, t1849: F, t925: F, t1814: F, t474: F, t763: F, t426: F, t1856: F, t431: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5474 = t473 * t1904;
    let t5477 = 0.003950778065781896 * t5474 * t483 * t485;
    let t5479 = t1910 * t1131 * t485;
    let t5495 = t142 * t1832;
    let t5502 = t1849 * t925;
    let t5504 = t1814 * t925;
    let t5506 = t474 * t763;
    let t5507 = t426 * t5506;
    let t5509 = t431 * t1856;
    (t5474, t5477, t5479, t5495, t5502, t5504, t5506, t5507, t5509)
}
