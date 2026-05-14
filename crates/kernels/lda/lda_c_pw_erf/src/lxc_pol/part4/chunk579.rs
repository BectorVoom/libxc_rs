//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 579/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk579<F: Float>(t133: F, t1655: F, t1663: F, t1717: F, t1868: F, t2598: F, t2601: F, t2613: F, t2616: F, t2620: F) -> (F,) {
    let t2642 = -t1655 + t2598 + t1663 + t2601 - t2613 + t1717 + 1.1495033333333333 * t1868 + 5.172765 * t133 * t2616 - 1.724255 * t133 * t2620;
    (t2642,)
}
