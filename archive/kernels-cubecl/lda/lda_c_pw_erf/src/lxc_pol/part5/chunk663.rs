//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 663/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk663<F: Float>(t1870: F, t1872: F, t5639: F, t1697: F, t9: F, t133: F, t5506: F, t5521: F, t1904: F, t285: F, t477: F, t281: F) -> (F, F, F, F, F, F) {
    let t5641 = t1870 * t5639 * t1872;
    let t5651 = t9 * t1697;
    let t5660 = t133 * t5506;
    let t5663 = F::cast_from(1.1495033333333333_f64) * t133 * t5521;
    let t5677 = t1904 * t477 * t285;
    let t5679 = F::cast_from(0.02394846802050922_f64) * t281 * t5677;
    (t5641, t5651, t5660, t5663, t5677, t5679)
}
