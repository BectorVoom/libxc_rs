//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 658/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk658<F: Float>(t1124: F, t780: F, t483: F, t485: F, t1904: F, t473: F, t1131: F, t1910: F, t142: F, t1832: F, t1849: F, t925: F) -> (F, F, F, F, F, F, F) {
    let t5470 = t1124 * t780;
    let t5472 = t5470 * t483 * t485;
    let t5474 = t473 * t1904;
    let t5477 = F::new(0.003950778065781896) * t5474 * t483 * t485;
    let t5479 = t1910 * t1131 * t485;
    let t5495 = t142 * t1832;
    let t5502 = t1849 * t925;
    (t5470, t5472, t5474, t5477, t5479, t5495, t5502)
}
